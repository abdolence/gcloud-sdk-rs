use crate::error::ErrorKind;
use hyper::http::uri::PathAndQuery;
use std::env;

#[derive(Debug)]
pub struct GceMetadataClient {
    metadata_client: Option<reqwest::Client>,
    metadata_server_host: String,
}

const GCE_METADATA_HOST_ENV: &str = "GCE_METADATA_HOST";

impl GceMetadataClient {
    pub fn new() -> Self {
        match env::var(GCE_METADATA_HOST_ENV).ok() {
            Some(metadata_server_host) => {
                let mut default_headers = reqwest::header::HeaderMap::new();
                default_headers.append(
                    "Metadata-Flavor",
                    "Google".parse().expect("Metadata-Flavor header is valid"),
                );
                default_headers.append(
                    reqwest::header::USER_AGENT,
                    crate::GCLOUD_SDK_USER_AGENT
                        .parse()
                        .expect("User agent header is valid"),
                );

                let metadata_client = reqwest::Client::builder()
                    .default_headers(default_headers)
                    .timeout(std::time::Duration::from_secs(5))
                    .tcp_keepalive(std::time::Duration::from_secs(60))
                    .build()
                    .ok();

                Self {
                    metadata_client,
                    metadata_server_host,
                }
            }
            None => Self {
                metadata_client: None,
                metadata_server_host: "metadata_server_host".into(),
            },
        }
    }

    pub fn is_available(&self) -> bool {
        self.metadata_client.is_some()
    }

    pub async fn get(&self, path_and_query: PathAndQuery) -> crate::error::Result<String> {
        match self.metadata_client {
            Some(ref client) => {
                let url = format!(
                    "http://{}/{}",
                    self.metadata_server_host,
                    path_and_query.as_str()
                );

                let response = client.get(&url).send().await?;

                let status = response.status();
                let body = response.text().await?;

                if status.is_success() {
                    Ok(body)
                } else {
                    Err(ErrorKind::Metadata(format!(
                        "Error retrieving data from metadata server: {}({}). URL: {}",
                        status, body, url
                    ))
                    .into())
                }
            }
            None => Err(ErrorKind::Metadata(format!(
                "Error retrieving data from metadata server: {}",
                "Metadata server not available"
            ))
            .into()),
        }
    }
}
