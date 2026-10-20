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
use hyper::header::{HeaderMap, HeaderName, HeaderValue};
use hyper::Uri;
use tokio::time::{sleep, sleep_until, Instant, Sleep};
use tonic::Status;
use tracing::error;

const GRPC_TIMEOUT: HeaderName = HeaderName::from_static("grpc-timeout");

/// The largest `grpc-timeout` value: the gRPC spec allows at most eight digits.
const MAX_GRPC_TIMEOUT_VALUE: u128 = 99_999_999;

/// The `grpc-timeout` units from the finest, each with its length in nanoseconds.
const GRPC_TIMEOUT_UNITS: [(char, u128); 6] = [
    ('n', 1),
    ('u', 1_000),
    ('m', 1_000_000),
    ('S', 1_000_000_000),
    ('M', 60 * 1_000_000_000),
    ('H', 60 * 60 * 1_000_000_000),
];

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
        if let Some(value) = grpc_timeout_value(remaining) {
            headers.entry(GRPC_TIMEOUT).or_insert(value);
        }
        Ok(())
    }

    /// Runs `send`, the call to `uri` up to its response headers, within the earlier of the
    /// two limits, and then bounds the response body by both.
    pub(crate) async fn limit<B: 'static, E: From<Status>>(
        self,
        uri: Uri,
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
            // The limit is polled first: with `total` set, tonic's channel races a deadline
            // read from `grpc-timeout`, no earlier than this one, and fails the call
            // `Cancelled`.
            Some((deadline, elapsed)) => {
                match select(pin!(sleep_until(deadline)), pin!(send)).await {
                    Either::Left(_) => {
                        error!(
                            req_uri = %uri,
                            "Err: took {}ms ({elapsed})",
                            self.started.elapsed().as_millis()
                        );
                        return Err(Status::from(elapsed).into());
                    }
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

/// `timeout` as a `grpc-timeout` value in the finest unit that fits, rounded up, so that a
/// deadline read from the header never comes before the one it was written from. `None`
/// past eight digits of hours.
fn grpc_timeout_value(timeout: Duration) -> Option<HeaderValue> {
    let nanos = timeout.as_nanos();
    let (value, unit) = GRPC_TIMEOUT_UNITS
        .iter()
        .map(|&(unit, unit_nanos)| (nanos.div_ceil(unit_nanos), unit))
        .find(|&(value, _)| value <= MAX_GRPC_TIMEOUT_VALUE)?;
    HeaderValue::try_from(format!("{value}{unit}")).ok()
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
                // A limit too large to add to the clock leaves the sleep at tokio's far
                // future, where `sleep(limit)` put it.
                if let Some(deadline) = Instant::now().checked_add(idle.limit) {
                    idle.sleep.as_mut().reset(deadline);
                }
                idle.waiting = true;
            }
        }
        match Pin::new(&mut this.inner).poll_frame(cx) {
            Poll::Ready(None) => {
                // A body polled again after its end must not fail on a limit.
                this.total = None;
                this.idle = None;
                Poll::Ready(None)
            }
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
    use std::future::poll_fn;
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

    /// Past 100 s tonic's own encoder drops the microseconds of `grpc-timeout`, and the
    /// channel would end the call on that shorter deadline, `Cancelled`.
    #[tokio::test(start_paused = true)]
    async fn total_limit_past_milliseconds_fails_with_deadline_exceeded() {
        let limit = Duration::from_micros(110_000_500);
        let (url, _) = silent_endpoint().await;
        let client: GoogleApi<GoogleAuthMiddleware> = GoogleApi::from_function_with_credentials(
            |service| service,
            url,
            None,
            Credentials::from(StubCredentials::bearer("token")),
        )
        .await
        .unwrap()
        .with_request_timeouts(RequestTimeouts::new().with_total(limit));

        let started = Instant::now();
        let status = unary(client.get(), tonic::Request::new(()))
            .await
            .unwrap_err();

        // tokio's timers fire on whole milliseconds.
        assert!((limit..limit + Duration::from_millis(1)).contains(&started.elapsed()));
        assert_eq!(status.code(), Code::DeadlineExceeded);
        assert_eq!(
            status.source().and_then(|source| source.downcast_ref()),
            Some(&RequestTimeoutElapsed::Total(limit))
        );
    }

    #[tokio::test(start_paused = true)]
    async fn total_limit_ends_a_stream_that_keeps_delivering() {
        let service = GoogleAuthMiddlewareService::new(
            stalling_stream_channel(vec![Duration::from_secs(2); 10]).await,
            GoogleAuthHeaders::from(Credentials::from(StubCredentials::bearer("token"))),
            None,
        )
        .unwrap();
        let mut request = tonic::Request::new(());
        request
            .extensions_mut()
            .insert(RequestTimeouts::new().with_total(LIMIT));

        let started = Instant::now();
        let mut messages = open_stream(service, request).await;
        let status = tokio::time::timeout(LIMIT * 2, async {
            while messages.message().await?.is_some() {}
            Ok::<_, Status>(())
        })
        .await
        .expect("the total limit ends the stream")
        .unwrap_err();

        assert_eq!(started.elapsed(), LIMIT);
        assert_eq!(status.code(), Code::DeadlineExceeded);
        assert_eq!(
            status.source().and_then(|source| source.downcast_ref()),
            Some(&RequestTimeoutElapsed::Total(LIMIT))
        );
    }

    /// A body that has ended stays ended, however long after its limits it is polled.
    #[tokio::test(start_paused = true)]
    async fn limits_end_with_the_response_body() {
        let clock = RequestTimeouts::new()
            .with_total(LIMIT)
            .with_idle(LIMIT)
            .start()
            .unwrap();
        let mut body = clock.limit_body(tonic::body::Body::new(String::from("message")));

        assert!(poll_fn(|cx| Pin::new(&mut body).poll_frame(cx))
            .await
            .is_some_and(|frame| frame.is_ok()));
        assert!(poll_fn(|cx| Pin::new(&mut body).poll_frame(cx))
            .await
            .is_none());
        tokio::time::sleep(LIMIT * 2).await;
        assert!(poll_fn(|cx| Pin::new(&mut body).poll_frame(cx))
            .await
            .is_none());
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
