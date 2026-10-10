//! The one metadata server call google-cloud-auth does not make: the project ID.

use std::time::Duration;

use tracing::*;

const METADATA_FLAVOR: &str = "metadata-flavor";

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

    /// The server at `root`, such as a stub server's URL.
    #[cfg(test)]
    pub(crate) fn at(root: String) -> Self {
        Self { root }
    }

    /// The ID of the project the instance runs in, or `None` when no metadata server
    /// answers with one.
    ///
    /// Only an answer that carries `Metadata-Flavor: Google` counts: anything else at
    /// that host, such as a captive portal or a proxy error page, is not the metadata
    /// server.
    pub(crate) async fn project_id(&self) -> Option<String> {
        let url = format!("{}/computeMetadata/v1/project/project-id", self.root);
        match self.fetch(&url).await {
            Ok(project_id) => project_id,
            Err(error) => {
                debug!(%url, %error, "The metadata server did not answer");
                None
            }
        }
    }

    async fn fetch(&self, url: &str) -> reqwest::Result<Option<String>> {
        let client = reqwest::Client::builder()
            .user_agent(crate::GCLOUD_SDK_USER_AGENT)
            .timeout(Duration::from_secs(5))
            .build()?;
        let response = client
            .get(url)
            .header(METADATA_FLAVOR, "Google")
            .send()
            .await?;
        let status = response.status();
        let flavor = response.headers().get(METADATA_FLAVOR);
        if !status.is_success() || flavor.is_none_or(|flavor| flavor != "Google") {
            debug!(%url, %status, ?flavor, "Not a metadata server answer");
            return Ok(None);
        }
        Ok(Some(response.text().await?.trim().to_string()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_support::{StubResponse, StubServer};

    #[tokio::test]
    async fn project_id_comes_from_the_metadata_server() {
        let server =
            StubServer::start(vec![StubResponse::text("200 OK", "my-project\n")
                .with_header("Metadata-Flavor", "Google")])
            .await;
        let metadata = MetadataServer::at(server.url.clone());

        assert_eq!(metadata.project_id().await.as_deref(), Some("my-project"));
        let received = server.received();
        assert_eq!(
            received[0].request_line,
            "GET /computeMetadata/v1/project/project-id HTTP/1.1"
        );
        assert_eq!(received[0].header("metadata-flavor"), Some("Google"));
    }

    #[tokio::test]
    async fn an_answer_without_the_metadata_flavor_is_not_a_project_id() {
        let server =
            StubServer::start(vec![StubResponse::text("200 OK", "<html>sign in</html>")]).await;
        let metadata = MetadataServer::at(server.url.clone());

        assert_eq!(metadata.project_id().await, None);
    }
}
