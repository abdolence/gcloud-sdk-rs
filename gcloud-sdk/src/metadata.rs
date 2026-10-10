//! The one metadata server call google-cloud-auth does not make: the project ID.

use std::time::Duration;

use crate::error::ErrorKind;

/// The metadata server of the Compute Engine instance, GKE node or Cloud Run instance the
/// process runs on.
pub(crate) struct MetadataServer {
    /// Scheme and host, such as `http://metadata.google.internal`.
    root: String,
}

impl MetadataServer {
    /// The server at `GCE_METADATA_HOST`, else at `metadata.google.internal`: where
    /// google-cloud-auth fetches its tokens.
    pub(crate) fn from_env() -> Self {
        let host = std::env::var("GCE_METADATA_HOST")
            .unwrap_or_else(|_| "metadata.google.internal".to_string());
        Self {
            root: format!("http://{host}"),
        }
    }

    /// The ID of the project the instance runs in.
    pub(crate) async fn project_id(&self) -> crate::error::Result<String> {
        let url = format!("{}/computeMetadata/v1/project/project-id", self.root);
        let client = reqwest::Client::builder()
            .user_agent(crate::GCLOUD_SDK_USER_AGENT)
            .timeout(Duration::from_secs(5))
            .build()?;
        let response = client
            .get(&url)
            .header("metadata-flavor", "Google")
            .send()
            .await?;
        let status = response.status();
        if !status.is_success() {
            return Err(ErrorKind::Metadata(format!("{url} answered {status}")).into());
        }
        Ok(response.text().await?.trim().to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_support::{StubResponse, StubServer};

    #[tokio::test]
    async fn project_id_comes_from_the_metadata_server() {
        let server = StubServer::start(vec![StubResponse::text("200 OK", "my-project\n")]).await;
        let metadata = MetadataServer {
            root: server.url.clone(),
        };

        assert_eq!(metadata.project_id().await.unwrap(), "my-project");
        let received = server.received();
        assert_eq!(
            received[0].request_line,
            "GET /computeMetadata/v1/project/project-id HTTP/1.1"
        );
        assert_eq!(received[0].header("metadata-flavor"), Some("Google"));
    }
}
