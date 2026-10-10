use std::{convert::From, fmt};

/// Represents the details of the [`Error`](struct.Error.html)
#[derive(Debug)]
#[non_exhaustive]
pub enum ErrorKind {
    /// Errors that can possibly occur while accessing an HTTP server.
    Http(reqwest::Error),
    /// An HTTP status other than 2xx.
    HttpStatus(reqwest::StatusCode),
    /// Credentials could not be built: no Application Default Credentials were found, or a
    /// credentials file is malformed or of a type the token cannot be minted from.
    CredentialsBuild(google_cloud_auth::build_errors::Error),
    /// Credentials failed to produce a token: a token endpoint refused them or could not be
    /// reached. [`CredentialsError::is_transient`](google_cloud_auth::errors::CredentialsError::is_transient)
    /// says whether a retry may succeed.
    Credentials(google_cloud_auth::errors::CredentialsError),
    /// GCE metadata service error.
    Metadata(String),
    TonicMetadata(tonic::metadata::errors::InvalidMetadataValue),
    GrpcStatus(tonic::transport::Error),
    UrlError(hyper::http::uri::InvalidUri),
    /// An API URL that parses but cannot be connected to: no scheme, a scheme
    /// other than `http` or `https`, or no host.
    InvalidApiUrl(String),
    /// A header value built from user input (user agent, headers, the token itself)
    /// failed HTTP header validation (e.g. contained a control character).
    HeaderValue(hyper::header::InvalidHeaderValue),
    /// The Application Default Credentials hold no service account to mint an ID token
    /// as. An ID token needs a service account to impersonate, given to
    /// [`GoogleAuthTokenGenerator::id_token_impersonating`](crate::GoogleAuthTokenGenerator::id_token_impersonating).
    IdTokenNeedsImpersonation(IdTokenUnsupportedCredentials),
    /// A `service_account_impersonation_url` in a credentials file that names no
    /// service account.
    InvalidImpersonationUrl(String),
}

/// Credentials that cannot mint an ID token without impersonating a service account
/// (see [`ErrorKind::IdTokenNeedsImpersonation`]).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum IdTokenUnsupportedCredentials {
    /// `authorized_user` credentials, such as those `gcloud auth application-default login`
    /// writes.
    AuthorizedUser,
    /// `external_account` credentials without a `service_account_impersonation_url`.
    ExternalAccount,
}

impl fmt::Display for IdTokenUnsupportedCredentials {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::AuthorizedUser => write!(f, "authorized_user credentials"),
            Self::ExternalAccount => write!(
                f,
                "external_account credentials without service_account_impersonation_url"
            ),
        }
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
            CredentialsBuild(ref e) => write!(f, "credentials build error: {}", e),
            Credentials(ref e) => write!(f, "credentials error: {}", e),
            Metadata(ref e) => write!(f, "gce metadata service error: {}", e),
            GrpcStatus(ref e) => write!(f, "Tonic/gRPC error: {}", e),
            TonicMetadata(ref e) => write!(f, "Tonic metadata error: {}", e),
            UrlError(ref e) => write!(f, "Url error: {}", e),
            InvalidApiUrl(ref e) => write!(f, "Invalid API URL: {}", e),
            HeaderValue(ref e) => write!(f, "invalid header value: {}", e),
            IdTokenNeedsImpersonation(ref credentials) => write!(
                f,
                "{} cannot mint an ID token; give a service account to impersonate",
                credentials
            ),
            InvalidImpersonationUrl(ref url) => write!(
                f,
                "service_account_impersonation_url names no service account: {}",
                url
            ),
        }
    }
}

impl std::error::Error for Error {}

impl From<reqwest::Error> for Error {
    fn from(e: reqwest::Error) -> Self {
        ErrorKind::Http(e).into()
    }
}

impl From<google_cloud_auth::build_errors::Error> for Error {
    fn from(e: google_cloud_auth::build_errors::Error) -> Self {
        ErrorKind::CredentialsBuild(e).into()
    }
}

impl From<google_cloud_auth::errors::CredentialsError> for Error {
    fn from(e: google_cloud_auth::errors::CredentialsError) -> Self {
        ErrorKind::Credentials(e).into()
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

impl From<hyper::header::InvalidHeaderValue> for Error {
    fn from(e: hyper::header::InvalidHeaderValue) -> Self {
        ErrorKind::HeaderValue(e).into()
    }
}

/// Wrapper for the `Result` type with an [`Error`](struct.Error.html).
pub type Result<T> = std::result::Result<T, Error>;
