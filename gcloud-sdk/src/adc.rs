//! The Application Default Credentials file, read for the two things google-cloud-auth
//! does not expose: the project it names, and its credential type, on which minting ID
//! tokens depends.

use std::path::PathBuf;

use serde_json::Value;
use tracing::*;

/// The JSON of the Application Default Credentials file, found where google-cloud-auth
/// looks for it.
pub(crate) struct AdcFile(Value);

/// What an [`AdcFile`] holds, as far as minting ID tokens is concerned.
pub(crate) enum AdcKind<'a> {
    /// User credentials, such as `gcloud auth application-default login` writes.
    AuthorizedUser,
    /// A workload or workforce identity federation configuration, as written.
    ExternalAccount(&'a Value),
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
            _ => AdcKind::Other,
        }
    }

    /// The project the file names: a service account key's `project_id`, else the
    /// `quota_project_id` of the credentials or, for impersonation, of their source
    /// credentials.
    pub(crate) fn project_id(&self) -> Option<&str> {
        self.0
            .get("project_id")
            .and_then(Value::as_str)
            .or_else(|| self.0.get("quota_project_id")?.as_str())
            .or_else(|| {
                self.0
                    .get("source_credentials")?
                    .get("quota_project_id")?
                    .as_str()
            })
    }
}

#[cfg(test)]
impl From<Value> for AdcFile {
    fn from(json: Value) -> Self {
        Self(json)
    }
}
