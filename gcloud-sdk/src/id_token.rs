use std::fmt;

use crate::error::ErrorKind;

/// The audience an ID token is minted for and checked against: the URL of a Cloud Run
/// service or a Cloud Run function, a custom audience configured on the service, or the
/// OAuth client ID of a resource behind Identity-Aware Proxy.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct IdTokenAudience(String);

impl IdTokenAudience {
    pub fn new(audience: impl Into<String>) -> Self {
        Self(audience.into())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for IdTokenAudience {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

/// The email of a service account, such as
/// `invoker@my-project.iam.gserviceaccount.com`.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ServiceAccountEmail(String);

impl ServiceAccountEmail {
    pub fn new(email: impl Into<String>) -> Self {
        Self(email.into())
    }

    /// The service account a `service_account_impersonation_url` names: the last path
    /// segment, up to the `:generateAccessToken` method.
    pub(crate) fn from_impersonation_url(url: &str) -> crate::error::Result<Self> {
        let resource = url.rsplit('/').next().unwrap_or_default();
        let email = resource.split(':').next().unwrap_or_default();
        if email.is_empty() {
            Err(ErrorKind::InvalidImpersonationUrl(url.to_string()).into())
        } else {
            Ok(Self::new(email))
        }
    }
}

impl fmt::Display for ServiceAccountEmail {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}
