//! The Application Default Credentials file, read for what google-cloud-auth does not
//! expose: the project it names, its credential type, on which minting ID tokens depends,
//! and whether it signs with a key of its own, which needs a rustls crypto provider.

use std::path::PathBuf;

use serde_json::Value;
use tracing::*;

use crate::error::ErrorKind;

/// The JSON of the Application Default Credentials file, found where google-cloud-auth
/// looks for it.
pub(crate) struct AdcFile(Value);

/// What an [`AdcFile`] holds, as far as minting ID tokens is concerned.
pub(crate) enum AdcKind<'a> {
    /// User credentials, such as `gcloud auth application-default login` writes.
    AuthorizedUser,
    /// A workload or workforce identity federation configuration, as written.
    ExternalAccount(&'a Value),
    /// A workforce identity federation user, such as `gcloud auth application-default
    /// login` writes for one.
    ExternalAccountAuthorizedUser,
    /// A Google Distributed Cloud Hosted service account.
    GdchServiceAccount,
    /// A service account key, impersonation, or a type google-cloud-auth reports itself.
    Other,
}

impl AdcFile {
    /// The file `GOOGLE_APPLICATION_CREDENTIALS` names, else the one gcloud writes.
    ///
    /// `None` when there is no such file or it is not JSON: building the credentials
    /// reports those cases, so this lookup never fails on its own.
    pub(crate) fn load() -> Option<Self> {
        let path = Self::path()?;
        let contents = match std::fs::read_to_string(&path) {
            Ok(contents) => contents,
            Err(error) => {
                debug!(?path, %error, "No readable Application Default Credentials file");
                return None;
            }
        };
        match serde_json::from_str(&contents) {
            Ok(json) => Some(Self(json)),
            Err(error) => {
                debug!(?path, %error, "The Application Default Credentials file is not JSON");
                None
            }
        }
    }

    fn path() -> Option<PathBuf> {
        std::env::var_os("GOOGLE_APPLICATION_CREDENTIALS")
            .map(PathBuf::from)
            .or_else(Self::well_known_path)
    }

    #[cfg(target_os = "windows")]
    fn well_known_path() -> Option<PathBuf> {
        std::env::var_os("APPDATA")
            .map(|root| PathBuf::from(root).join("gcloud/application_default_credentials.json"))
    }

    #[cfg(not(target_os = "windows"))]
    fn well_known_path() -> Option<PathBuf> {
        std::env::var_os("HOME").map(|home| {
            PathBuf::from(home).join(".config/gcloud/application_default_credentials.json")
        })
    }

    pub(crate) fn kind(&self) -> AdcKind<'_> {
        match self.0.get("type").and_then(Value::as_str) {
            Some("authorized_user") => AdcKind::AuthorizedUser,
            Some("external_account") => AdcKind::ExternalAccount(&self.0),
            Some("external_account_authorized_user") => AdcKind::ExternalAccountAuthorizedUser,
            Some("gdch_service_account") => AdcKind::GdchServiceAccount,
            _ => AdcKind::Other,
        }
    }

    /// Fails with [`ErrorKind::CryptoProviderMissing`] when these credentials sign with a
    /// key of their own and google-cloud-auth has no rustls crypto provider to sign with.
    ///
    /// Call it before building credentials from this file: google-cloud-auth signs in the
    /// refresh task it spawns at build, and panics there, and in every later token
    /// request, when no provider is installed.
    pub(crate) fn ensure_signing_provider(&self) -> crate::error::Result<()> {
        if cfg!(feature = "auth-default-crypto")
            || !self.signs_locally()
            || rustls::crypto::CryptoProvider::get_default().is_some()
        {
            Ok(())
        } else {
            Err(ErrorKind::CryptoProviderMissing.into())
        }
    }

    /// A service account key, used directly or as the source of an impersonation.
    fn signs_locally(&self) -> bool {
        let is_key = |credentials: &Value| {
            credentials.get("type").and_then(Value::as_str) == Some("service_account")
        };
        is_key(&self.0) || self.0.get("source_credentials").is_some_and(is_key)
    }

    /// The project the file names: its `project_id`, else its `quota_project_id`, else,
    /// for impersonation, the `project_id` and then the `quota_project_id` of its source
    /// credentials.
    pub(crate) fn project_id(&self) -> Option<&str> {
        project_of(&self.0).or_else(|| project_of(self.0.get("source_credentials")?))
    }
}

/// The `project_id` of `credentials`, else their `quota_project_id`.
fn project_of(credentials: &Value) -> Option<&str> {
    credentials
        .get("project_id")
        .and_then(Value::as_str)
        .or_else(|| credentials.get("quota_project_id")?.as_str())
}

#[cfg(test)]
impl From<Value> for AdcFile {
    fn from(json: Value) -> Self {
        Self(json)
    }
}
