//! Fixtures shared by the tests: an RSA key and a local HTTP stub server.

// Without a JWT crypto provider only the missing-provider errors are tested, and most
// fixtures go unused.
#![cfg_attr(
    not(any(feature = "jwt-aws-lc-rs", feature = "jwt-rust-crypto")),
    allow(dead_code)
)]

use std::sync::{Arc, Mutex};

use jsonwebtoken::{encode, Algorithm, EncodingKey, Header};
use tokio::io::{AsyncReadExt, AsyncWriteExt};

/// A 2048-bit RSA key generated for these tests; it signs nothing outside them.
pub(crate) const TEST_RSA_PRIVATE_KEY: &str = r"-----BEGIN PRIVATE KEY-----
MIIEvQIBADANBgkqhkiG9w0BAQEFAASCBKcwggSjAgEAAoIBAQDfuSBdVqRqLKTk
uFNL6aieALaeza3kcVXFzIMZw+SIXWMW372Hes+ettM5501atS2ND8FXSmP3NR9/
8ac4kINZLlt94Y2l2uu0NCagKYWRxGd96oYpywYNLJQWvPxv9ETO1tVeDlT/EQ3f
sLlRmhSvZBjS3mWf2lX7AkvLz6zR/h4rvREwoNbz6q8Go9hdsxTNFsQcbIE6ZhI2
75hZxR0rKZA0n8UUA552HoAa3OJx03iGABK7h/k3hm7ZmnPaD558RVHUc7imJwsz
5uMpoIiTzBJlewvI68OkVdfuvM8UVeItVGj1t3cgg0ClAOfuVxn83c1iNTBMobQ3
IUc9u8mBAgMBAAECggEAAZB3kbeCoExuHbxNNs1sNKigHkWlZuDE/wRSUVqNjUeL
4xPO7TTWYU95dCDyKUV1i1Q2H6BhSQ/5x6j+qJZYGMZKdPugBC4e8kxgDcQkjzOe
nqKGbCHjibGLWopZQIYmegTGCqmSfhMWM/92GEQ5y00poEXWU5MRAVePhJ4P4QsA
vVKVB408BdYl7/ll+N9jNNICKdlX8ETrBp+TPTi2VmGuCPSRGH9EUehmzxuRwUoK
U4uY10En8pyEMBjiNGiOjyFn6NvV4KtsMyUIh2DrUBEcyAYSRt33Lzcfflf3dS4p
6IN6uNFzPhtFBY4kKO01w7ssabBHO3bWhZ61PMvjAQKBgQD2yIVDvFegCW/t9/4i
VBUJP08XH7fKU3L1Y6hZnJodzsYZl4wYDP8j5uqcbsc5aocv5bAWDQugxK1OrBp3
NgozQiOjkIsPjS1Od6GGMIyDXvprki9iWyYnkfi1aiKS8+ZI2rN+xUeyBkBHGzcz
Nxqju1E1pa6ziSfCjXQfGu40IQKBgQDoFCEezWHknZHss/V853qG5hDeFooYvfgd
gB+ErvMZkJzFcmxVJPtc/j/rNXCoIT/tzFlEsLjYgjskBlrOzuQOzlUNySYejN3g
uwX/3icMDrurf3DsXHVGf2hUrNjqECrV+gGRnfmVBvtBs5hswIbLe4sdnARd5w0D
+2PGOwjpYQKBgGQ5W6X+v7eHHaYPqW5Xp5Nx2rURdJr++RkfuCdsqkqgx2NtYMAD
xzrVdULC2rY+xVh2d+T8t1Q7jAb/bmAr2kim+8JZ2aAfPd84Rqkw3mAcGqfFXukb
C4vWhKNoz1HPLB86cttxU4TBdSlrrCdoobENShX3i9PuR++Dcz2Oul8BAoGAHYbC
F/slY0Kw2B6lMvj4W8VVjAvuEevJb2dnmyfvAeemKnC+W67S1Cf81d38sUdZrNV8
3gJl4hXflFvCneEwnrmdlJ1s3iIp8Hea8cy/xwbw1YbnRQsWWJvJGEzNZCoeaQ7f
uDkTEeTLfrZsxBlSjPzw3BmHbLMCsuj+7q+AGMECgYEAp95XH5iyxxsqDRbtsDD3
xcOguBDd9RXvGAI0v2ZdC1CF/MLsHRT8+COxaVb60NRRiyVTbyQ2BDfR5B6Tb8L1
qv0DqY+7mrGKzFdVr3Uf2lJJYFipabaL1GrD5FkZYPW21eNfBj2QRorgLf70n7L+
VPNvPfZ32mJr45YUokuFcGU=
-----END PRIVATE KEY-----";

/// A JWT signed with [`TEST_RSA_PRIVATE_KEY`] under key ID `kid`.
pub(crate) fn signed_jwt(kid: &str, claims: &serde_json::Value) -> String {
    let header = Header {
        kid: Some(kid.to_string()),
        ..Header::new(Algorithm::RS256)
    };
    crate::jwt_crypto::ensure_provider().unwrap();
    let key = EncodingKey::from_rsa_pem(TEST_RSA_PRIVATE_KEY.as_bytes()).unwrap();
    encode(&header, claims, &key).unwrap()
}

/// A JWK set that publishes the public half of [`TEST_RSA_PRIVATE_KEY`] under each of
/// `key_ids`.
#[cfg(feature = "id-token-verify")]
pub(crate) fn test_jwk_set(key_ids: &[&str]) -> jsonwebtoken::jwk::JwkSet {
    crate::jwt_crypto::ensure_provider().unwrap();
    let encoding_key = EncodingKey::from_rsa_pem(TEST_RSA_PRIVATE_KEY.as_bytes()).unwrap();
    let keys = key_ids
        .iter()
        .map(|kid| {
            let mut jwk =
                jsonwebtoken::jwk::Jwk::from_encoding_key(&encoding_key, Algorithm::RS256).unwrap();
            jwk.common.key_id = Some(kid.to_string());
            jwk
        })
        .collect();
    jsonwebtoken::jwk::JwkSet { keys }
}

/// One response of a [`StubServer`].
pub(crate) struct StubResponse {
    status_line: &'static str,
    headers: Vec<(&'static str, String)>,
    body: String,
}

impl StubResponse {
    pub(crate) fn json(status_line: &'static str, body: impl Into<String>) -> Self {
        Self {
            status_line,
            headers: vec![("content-type", "application/json".to_string())],
            body: body.into(),
        }
    }

    #[cfg(feature = "id-token-verify")]
    pub(crate) fn with_header(mut self, name: &'static str, value: impl Into<String>) -> Self {
        self.headers.push((name, value.into()));
        self
    }
}

/// A request a [`StubServer`] received.
#[derive(Debug, Clone)]
pub(crate) struct ReceivedRequest {
    /// The request line, such as `POST /token HTTP/1.1`.
    pub(crate) request_line: String,
    pub(crate) headers: Vec<(String, String)>,
    pub(crate) body: String,
}

impl ReceivedRequest {
    pub(crate) fn header(&self, name: &str) -> Option<&str> {
        self.headers
            .iter()
            .find(|(header, _)| header.eq_ignore_ascii_case(name))
            .map(|(_, value)| value.as_str())
    }
}

/// A local HTTP/1.1 server that answers one request per connection with the given
/// responses in order, records every request, and stops accepting after the last one.
pub(crate) struct StubServer {
    pub(crate) url: String,
    received: Arc<Mutex<Vec<ReceivedRequest>>>,
}

impl StubServer {
    pub(crate) async fn start(responses: Vec<StubResponse>) -> Self {
        let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
        let url = format!("http://{}", listener.local_addr().unwrap());
        let received = Arc::new(Mutex::new(Vec::new()));
        let recorder = Arc::clone(&received);
        tokio::spawn(async move {
            for response in responses {
                let (mut socket, _) = listener.accept().await.unwrap();
                let request = read_request(&mut socket).await;
                recorder.lock().unwrap().push(request);
                let mut head = format!(
                    "HTTP/1.1 {}\r\ncontent-length: {}\r\nconnection: close\r\n",
                    response.status_line,
                    response.body.len()
                );
                for (name, value) in &response.headers {
                    head.push_str(&format!("{name}: {value}\r\n"));
                }
                head.push_str("\r\n");
                socket.write_all(head.as_bytes()).await.unwrap();
                socket.write_all(response.body.as_bytes()).await.unwrap();
            }
        });
        Self { url, received }
    }

    pub(crate) fn received(&self) -> Vec<ReceivedRequest> {
        self.received.lock().unwrap().clone()
    }
}

async fn read_request(socket: &mut tokio::net::TcpStream) -> ReceivedRequest {
    let mut buf = Vec::new();
    let mut chunk = [0u8; 4096];
    let head_end = loop {
        let read = socket.read(&mut chunk).await.unwrap();
        assert!(read > 0, "connection closed before the request head ended");
        buf.extend_from_slice(&chunk[..read]);
        if let Some(pos) = buf.windows(4).position(|window| window == b"\r\n\r\n") {
            break pos + 4;
        }
    };
    let head = String::from_utf8(buf[..head_end].to_vec()).unwrap();
    let mut lines = head.split("\r\n").filter(|line| !line.is_empty());
    let request_line = lines.next().unwrap().to_string();
    let headers: Vec<(String, String)> = lines
        .map(|line| {
            let (name, value) = line.split_once(':').unwrap();
            (name.trim().to_string(), value.trim().to_string())
        })
        .collect();
    let content_length: usize = headers
        .iter()
        .find(|(name, _)| name.eq_ignore_ascii_case("content-length"))
        .map(|(_, value)| value.parse().unwrap())
        .unwrap_or(0);
    while buf.len() < head_end + content_length {
        let read = socket.read(&mut chunk).await.unwrap();
        assert!(read > 0, "connection closed before the request body ended");
        buf.extend_from_slice(&chunk[..read]);
    }
    let body = String::from_utf8(buf[head_end..head_end + content_length].to_vec()).unwrap();
    ReceivedRequest {
        request_line,
        headers,
        body,
    }
}
