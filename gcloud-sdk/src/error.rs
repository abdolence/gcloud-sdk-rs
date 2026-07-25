use std::{convert::From, fmt};

/// Represents the details of the [`Error`](struct.Error.html)
#[derive(Debug)]
pub enum ErrorKind {
    /// Errors that can possibly occur while accessing an HTTP server.
    Http(reqwest::Error),
    /// Http status code that is not 2xx when getting token.
    HttpStatus(reqwest::StatusCode),
    /// Authentication failed while obtaining or refreshing a token.
    /// Distinguishes auth failures from errors of the API call itself.
    Auth(AuthErrorDetails),
    /// GCE metadata service error.
    Metadata(String),
    TonicMetadata(tonic::metadata::errors::InvalidMetadataValue),
    /// JWT encode/decode error.
    Jwt(jsonwebtoken::errors::Error),
    /// Token source error.
    TokenSource,
    /// An error parsing credentials file.
    CredentialsJson(serde_json::Error),
    /// An error reading credentials file.
    CredentialsFile(std::io::Error),
    /// An error from json serialization and deserialization.
    TokenJson(serde_json::Error),
    /// Invalid token error.
    TokenData,
    GrpcStatus(tonic::transport::Error),
    UrlError(hyper::http::uri::InvalidUri),
    ExternalCredsSourceError(String),
    #[doc(hidden)]
    __Nonexhaustive,
}

/// Details of an authentication failure (see [`ErrorKind::Auth`]).
#[derive(Debug)]
pub struct AuthErrorDetails {
    /// HTTP status returned by the authentication endpoint, if any.
    pub status: Option<reqwest::StatusCode>,
    /// OAuth error code from the response body (e.g. `invalid_grant`), if present.
    pub oauth_error: Option<String>,
    /// Human readable details: the OAuth `error_description` or the raw response body.
    pub details: Option<String>,
}

impl AuthErrorDetails {
    /// A remediation hint for well-known OAuth error codes, if one is available.
    pub fn hint(&self) -> Option<&'static str> {
        match self.oauth_error.as_deref() {
            Some("invalid_grant") => Some(
                "the credentials are likely expired or revoked; re-authenticate (e.g. `gcloud auth application-default login`) or provide a new service account key",
            ),
            _ => None,
        }
    }
}

impl fmt::Display for AuthErrorDetails {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut separate = false;
        if let Some(ref status) = self.status {
            write!(f, "HTTP {}", status)?;
            separate = true;
        }
        if let Some(ref e) = self.oauth_error {
            if separate {
                write!(f, ", ")?;
            }
            write!(f, "oauth error: {}", e)?;
            separate = true;
        }
        if let Some(ref d) = self.details {
            if separate {
                write!(f, " ")?;
            }
            write!(f, "({})", d)?;
        }
        if let Some(hint) = self.hint() {
            write!(f, ". Hint: {}", hint)?;
        }
        Ok(())
    }
}

/// Represents errors that can occur during getting token.
#[derive(Debug)]
pub struct Error(Box<ErrorKind>);

impl Error {
    /// Borrow [`ErrorKind`](enum.ErrorKind.html).
    pub fn kind(&self) -> &ErrorKind {
        &self.0
    }

    /// To own [`ErrorKind`](enum.ErrorKind.html).
    pub fn into_kind(self) -> ErrorKind {
        *self.0
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use ErrorKind::*;
        match *self.0 {
            Http(ref e) => write!(f, "http error: {}", e),
            HttpStatus(ref s) => write!(f, "http status error: {}", s),
            Auth(ref details) => write!(f, "authentication error: {}", details),
            Metadata(ref e) => write!(f, "gce metadata service error: {}", e),
            Jwt(ref e) => write!(f, "jwt error: {}", e),
            TokenSource => write!(f, "token source error: not found token source"),
            CredentialsJson(ref e) => write!(f, "credentials json error: {}", e),
            CredentialsFile(ref e) => write!(f, "credentials file error: {}", e),
            TokenJson(ref e) => write!(f, "token json error: {}", e),
            TokenData => write!(f, "token data error: invalid token response data"),
            GrpcStatus(ref e) => write!(f, "Tonic/gRPC error: {}", e),
            TonicMetadata(ref e) => write!(f, "Tonic metadata error: {}", e),
            UrlError(ref e) => write!(f, "Url error: {}", e),
            ExternalCredsSourceError(ref e) => write!(f, "External creds source error: {}", e),
            __Nonexhaustive => write!(f, "unknown error"),
        }
    }
}

impl std::error::Error for Error {}

impl From<reqwest::Error> for Error {
    fn from(e: reqwest::Error) -> Self {
        ErrorKind::Http(e).into()
    }
}

impl From<jsonwebtoken::errors::Error> for Error {
    fn from(e: jsonwebtoken::errors::Error) -> Self {
        ErrorKind::Jwt(e).into()
    }
}

impl From<ErrorKind> for Error {
    fn from(k: ErrorKind) -> Self {
        Error(Box::new(k))
    }
}

impl From<tonic::transport::Error> for Error {
    fn from(e: tonic::transport::Error) -> Self {
        ErrorKind::GrpcStatus(e).into()
    }
}

impl From<tonic::metadata::errors::InvalidMetadataValue> for Error {
    fn from(e: tonic::metadata::errors::InvalidMetadataValue) -> Self {
        ErrorKind::TonicMetadata(e).into()
    }
}

impl From<hyper::http::uri::InvalidUri> for Error {
    fn from(e: hyper::http::uri::InvalidUri) -> Self {
        ErrorKind::UrlError(e).into()
    }
}

/// Wrapper for the `Result` type with an [`Error`](struct.Error.html).
pub type Result<T> = std::result::Result<T, Error>;
