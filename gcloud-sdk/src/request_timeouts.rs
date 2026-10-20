use std::any::Any;
use std::fmt;
use std::future::Future;
use std::pin::{pin, Pin};
use std::sync::Arc;
use std::task::{Context, Poll};
use std::time::Duration;

use bytes::Bytes;
use futures::future::{select, Either};
use hyper::body::{Body, Frame, SizeHint};
use hyper::header::{HeaderMap, HeaderName};
use tokio::time::{sleep, sleep_until, Instant, Sleep};
use tonic::Status;

const GRPC_TIMEOUT: HeaderName = HeaderName::from_static("grpc-timeout");

/// The longest timeout `grpc-timeout` can carry: eight digits of hours.
const MAX_GRPC_TIMEOUT: Duration = Duration::from_secs(99_999_999 * 60 * 60);

/// Client-side time limits for a gRPC call. Both are off by default.
///
/// Set them for every call of a client with
/// [`GoogleApiClient::with_request_timeouts`](crate::GoogleApiClient::with_request_timeouts),
/// or for one call by inserting a value into the request's extensions, which replaces both
/// limits of the client for that call, so `RequestTimeouts::new()` switches them off:
///
/// ```rust,no_run
/// # use std::time::Duration;
/// # use gcloud_sdk::RequestTimeouts;
/// # fn sketch<M>(message: M) {
/// let mut request: tonic::Request<M> = tonic::Request::new(message);
/// request
///     .extensions_mut()
///     .insert(RequestTimeouts::new().with_total(Duration::from_secs(10)));
/// # }
/// ```
///
/// A call that runs out of a limit fails with
/// [`Code::DeadlineExceeded`](tonic::Code::DeadlineExceeded), and the status's `source()` is
/// a [`RequestTimeoutElapsed`].
///
/// The limits bound the response body only when the service under the middleware answers
/// with a [`tonic::body::Body`], as the channel of a
/// [`GoogleAuthMiddleware`](crate::GoogleAuthMiddleware) does. Over any other service they
/// bound the wait for the response headers alone.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct RequestTimeouts {
    total: Option<Duration>,
    idle: Option<Duration>,
}

impl RequestTimeouts {
    /// No limits: a call waits for as long as its connection lives.
    pub const fn new() -> Self {
        Self {
            total: None,
            idle: None,
        }
    }

    /// The longest the whole call may take, from the request, its authentication included,
    /// to the last response message and the trailers.
    ///
    /// Sent to the server as `grpc-timeout`, so a server that honours it stops working on
    /// the call too; a `grpc-timeout` the request already carries is kept. It ends a stream
    /// however steadily the stream delivers, so it suits unary calls.
    pub const fn with_total(self, total: Duration) -> Self {
        Self {
            total: Some(total),
            ..self
        }
    }

    /// The longest the caller waits for the response to start and for each next part of
    /// the response, counted only while the caller is waiting for one. Not sent to the
    /// server.
    ///
    /// It suits server streams. A stream that stays quiet until the client writes, such as a
    /// bidirectional stream, fails on it.
    pub const fn with_idle(self, idle: Duration) -> Self {
        Self {
            idle: Some(idle),
            ..self
        }
    }

    pub(crate) fn start(self) -> Option<CallClock> {
        if self.total.is_none() && self.idle.is_none() {
            return None;
        }
        let started = Instant::now();
        Some(CallClock {
            started,
            total: self
                .total
                .and_then(|limit| Some((limit, started.checked_add(limit)?))),
            idle: self.idle,
        })
    }
}

/// The limit of a [`RequestTimeouts`] that a call ran out of: the source of the
/// [`Code::DeadlineExceeded`](tonic::Code::DeadlineExceeded) status the call fails with.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum RequestTimeoutElapsed {
    /// The total limit, set with [`RequestTimeouts::with_total`].
    Total(Duration),
    /// The idle limit, set with [`RequestTimeouts::with_idle`].
    Idle(Duration),
}

impl fmt::Display for RequestTimeoutElapsed {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Total(limit) => write!(f, "total request timeout of {limit:?} elapsed"),
            Self::Idle(limit) => write!(f, "idle request timeout of {limit:?} elapsed"),
        }
    }
}

impl std::error::Error for RequestTimeoutElapsed {}

impl From<RequestTimeoutElapsed> for Status {
    fn from(elapsed: RequestTimeoutElapsed) -> Self {
        let mut status = Status::deadline_exceeded(elapsed.to_string());
        status.set_source(Arc::new(elapsed));
        status
    }
}

/// The limits of one call, measured from the moment it entered the middleware. A limit too
/// large to add to that moment is no limit.
#[derive(Clone, Copy)]
pub(crate) struct CallClock {
    started: Instant,
    /// The total limit and the moment it runs out.
    total: Option<(Duration, Instant)>,
    idle: Option<Duration>,
}

impl CallClock {
    /// Sends the time left of the total limit as `grpc-timeout`, unless the request carries
    /// a `grpc-timeout` of its own, and fails the call when no time is left.
    pub(crate) fn insert_grpc_timeout(&self, headers: &mut HeaderMap) -> Result<(), Status> {
        let Some((limit, deadline)) = self.total else {
            return Ok(());
        };
        let remaining = deadline.saturating_duration_since(Instant::now());
        if remaining.is_zero() {
            return Err(RequestTimeoutElapsed::Total(limit).into());
        }
        // tonic's encoder panics past the eight digits, and it is reachable only through
        // `tonic::Request::set_timeout`.
        if remaining <= MAX_GRPC_TIMEOUT {
            let mut encoder = tonic::Request::new(());
            encoder.set_timeout(remaining);
            if let Some(value) = encoder.into_parts().0.into_headers().remove(GRPC_TIMEOUT) {
                headers.entry(GRPC_TIMEOUT).or_insert(value);
            }
        }
        Ok(())
    }

    /// Runs `send`, the call up to its response headers, within the earlier of the two
    /// limits, and then bounds the response body by both.
    pub(crate) async fn limit<B: 'static, E: From<Status>>(
        self,
        send: impl Future<Output = Result<hyper::Response<B>, E>>,
    ) -> Result<hyper::Response<B>, E> {
        let total = self
            .total
            .map(|(limit, deadline)| (deadline, RequestTimeoutElapsed::Total(limit)));
        let idle = self.idle.and_then(|limit| {
            Some((
                self.started.checked_add(limit)?,
                RequestTimeoutElapsed::Idle(limit),
            ))
        });
        let response = match total.into_iter().chain(idle).min_by_key(|(at, _)| *at) {
            // The limit is polled first: with `total` set, tonic's channel races about the
            // same deadline, read from `grpc-timeout`, and fails the call `Cancelled`.
            Some((deadline, elapsed)) => {
                match select(pin!(sleep_until(deadline)), pin!(send)).await {
                    Either::Left(_) => return Err(Status::from(elapsed).into()),
                    Either::Right((response, _)) => response?,
                }
            }
            None => send.await?,
        };
        Ok(response.map(|body| self.limit_body(body)))
    }

    /// The response keeps the inner service's body type, so only the `tonic::body::Body` a
    /// tonic channel answers with can be replaced by a limited one.
    fn limit_body<B: 'static>(self, mut body: B) -> B {
        if let Some(body) = (&mut body as &mut dyn Any).downcast_mut::<tonic::body::Body>() {
            *body = tonic::body::Body::new(TimeoutBody {
                inner: std::mem::take(body),
                total: self
                    .total
                    .map(|(limit, deadline)| (limit, Box::pin(sleep_until(deadline)))),
                idle: self.idle.map(|limit| IdleTimer {
                    limit,
                    sleep: Box::pin(sleep(limit)),
                    waiting: false,
                }),
            });
        }
        body
    }
}

struct TimeoutBody {
    inner: tonic::body::Body,
    /// The total limit and the sleep until it runs out.
    total: Option<(Duration, Pin<Box<Sleep>>)>,
    idle: Option<IdleTimer>,
}

/// The idle limit of a response body, armed when the caller starts waiting for a frame and
/// disarmed when one arrives, so that a caller busy between frames never trips it.
struct IdleTimer {
    limit: Duration,
    sleep: Pin<Box<Sleep>>,
    waiting: bool,
}

impl Body for TimeoutBody {
    type Data = Bytes;
    type Error = Status;

    fn poll_frame(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
    ) -> Poll<Option<Result<Frame<Bytes>, Status>>> {
        let this = self.get_mut();
        if let Some((limit, deadline)) = &mut this.total {
            if deadline.as_mut().poll(cx).is_ready() {
                return Poll::Ready(Some(Err(RequestTimeoutElapsed::Total(*limit).into())));
            }
        }
        if let Some(idle) = &mut this.idle {
            if !idle.waiting {
                idle.sleep.set(sleep(idle.limit));
                idle.waiting = true;
            }
        }
        match Pin::new(&mut this.inner).poll_frame(cx) {
            Poll::Ready(frame) => {
                if let Some(idle) = &mut this.idle {
                    idle.waiting = false;
                }
                Poll::Ready(frame)
            }
            Poll::Pending => {
                if let Some(idle) = &mut this.idle {
                    if idle.sleep.as_mut().poll(cx).is_ready() {
                        return Poll::Ready(Some(Err(
                            RequestTimeoutElapsed::Idle(idle.limit).into()
                        )));
                    }
                }
                Poll::Pending
            }
        }
    }

    fn is_end_stream(&self) -> bool {
        self.inner.is_end_stream()
    }

    fn size_hint(&self) -> SizeHint {
        self.inner.size_hint()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::middleware::GoogleAuthMiddlewareService;
    use crate::test_support::{silent_endpoint, stalling_stream_channel, StubCredentials};
    use crate::{GoogleApi, GoogleAuthHeaders, GoogleAuthMiddleware};
    use google_cloud_auth::credentials::Credentials;
    use hyper::http::uri::PathAndQuery;
    use std::error::Error as _;
    use tokio::time::Instant;
    use tonic::client::Grpc;
    use tonic::codec::Streaming;
    use tonic::Code;
    use tonic_prost::ProstCodec;

    const LIMIT: Duration = Duration::from_secs(5);

    async fn unary(
        service: GoogleAuthMiddleware,
        request: tonic::Request<()>,
    ) -> Result<tonic::Response<()>, Status> {
        let mut grpc = Grpc::new(service);
        grpc.ready().await.unwrap();
        grpc.unary(
            request,
            PathAndQuery::from_static("/test.Silent/Call"),
            ProstCodec::default(),
        )
        .await
    }

    async fn open_stream(
        service: GoogleAuthMiddleware,
        request: tonic::Request<()>,
    ) -> Streaming<()> {
        let mut grpc = Grpc::new(service);
        grpc.ready().await.unwrap();
        grpc.server_streaming(
            request,
            PathAndQuery::from_static("/test.StallingStream/Stream"),
            ProstCodec::default(),
        )
        .await
        .unwrap()
        .into_inner()
    }

    #[tokio::test(start_paused = true)]
    async fn total_limit_fails_a_call_the_server_never_answers() {
        let (url, _) = silent_endpoint().await;
        let client: GoogleApi<GoogleAuthMiddleware> = GoogleApi::from_function_with_credentials(
            |service| service,
            url,
            None,
            Credentials::from(StubCredentials::bearer("token")),
        )
        .await
        .unwrap()
        .with_request_timeouts(RequestTimeouts::new().with_total(LIMIT));

        let started = Instant::now();
        let status = unary(client.get(), tonic::Request::new(()))
            .await
            .unwrap_err();

        assert_eq!(started.elapsed(), LIMIT);
        assert_eq!(status.code(), Code::DeadlineExceeded);
        assert_eq!(
            status.source().and_then(|source| source.downcast_ref()),
            Some(&RequestTimeoutElapsed::Total(LIMIT))
        );
    }

    #[tokio::test(start_paused = true)]
    async fn idle_limit_fails_a_stream_that_stops_after_its_first_message() {
        let service = GoogleAuthMiddlewareService::new(
            stalling_stream_channel(vec![Duration::ZERO]).await,
            GoogleAuthHeaders::from(Credentials::from(StubCredentials::bearer("token"))),
            None,
        )
        .unwrap();
        let mut request = tonic::Request::new(());
        request
            .extensions_mut()
            .insert(RequestTimeouts::new().with_idle(LIMIT));
        let mut messages = open_stream(service, request).await;
        assert_eq!(messages.message().await.unwrap(), Some(()));

        let started = Instant::now();
        let status = tokio::time::timeout(LIMIT * 2, messages.message())
            .await
            .expect("the idle limit ends the wait")
            .unwrap_err();

        assert_eq!(started.elapsed(), LIMIT);
        assert_eq!(status.code(), Code::DeadlineExceeded);
        assert_eq!(
            status.source().and_then(|source| source.downcast_ref()),
            Some(&RequestTimeoutElapsed::Idle(LIMIT))
        );
    }

    /// The idle clock runs only while the caller waits: no wait here is longer than 3 s,
    /// although the stream runs for 13 s and the server is silent for 7 s while the reader
    /// is busy. The request's limits replace the service's total limit.
    #[tokio::test(start_paused = true)]
    async fn idle_limit_spares_a_slow_stream_and_a_slow_reader() {
        let gaps = [3, 3, 7].map(Duration::from_secs).to_vec();
        let mut service = GoogleAuthMiddlewareService::new(
            stalling_stream_channel(gaps).await,
            GoogleAuthHeaders::from(Credentials::from(StubCredentials::bearer("token"))),
            None,
        )
        .unwrap();
        service.set_request_timeouts(RequestTimeouts::new().with_total(LIMIT));
        let mut request = tonic::Request::new(());
        request
            .extensions_mut()
            .insert(RequestTimeouts::new().with_idle(LIMIT));
        let mut messages = open_stream(service, request).await;

        assert_eq!(messages.message().await.unwrap(), Some(()));
        assert_eq!(messages.message().await.unwrap(), Some(()));
        tokio::time::sleep(Duration::from_secs(4)).await;
        assert_eq!(messages.message().await.unwrap(), Some(()));
    }

    /// Nothing set, nothing limited: a call to a server that never answers is still waiting
    /// a minute later, when only the connection's keep-alive can end it.
    #[tokio::test(start_paused = true)]
    async fn without_limits_a_call_waits_for_the_connection() {
        let (url, _) = silent_endpoint().await;
        let client: GoogleApi<GoogleAuthMiddleware> = GoogleApi::from_function_with_credentials(
            |service| service,
            url,
            None,
            Credentials::from(StubCredentials::bearer("token")),
        )
        .await
        .unwrap();

        let call = unary(client.get(), tonic::Request::new(()));

        assert!(tokio::time::timeout(Duration::from_secs(60), call)
            .await
            .is_err());
    }
}
