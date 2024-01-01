use url::form_urlencoded::Serializer;

use async_trait::async_trait;
use hyper::http::uri::PathAndQuery;
use secret_vault_value::SecretValue;
use std::convert::TryFrom;
use std::str::FromStr;
use tracing::trace;

use crate::token_source::{BoxSource, Source, Token, TokenResponse};

#[derive(Debug)]
pub struct Metadata {
    account: String,
    scopes: Vec<String>,
    gcemeta_client: gcemeta::Client<hyper::client::connect::HttpConnector, hyper::Body>,
}

impl Metadata {
    pub fn new(scopes: impl Into<Vec<String>>) -> Self {
        Self::with_account(scopes, "default".to_string())
    }

    pub fn with_account(scopes: impl Into<Vec<String>>, account: String) -> Self {
        Self {
            account,
            scopes: scopes.into(),
            gcemeta_client: gcemeta::Client::new(),
        }
    }

    fn uri_suffix(&self) -> String {
        let query = if self.scopes.is_empty() {
            String::new()
        } else {
            Serializer::new(String::new())
                .append_pair("scopes", &self.scopes.join(","))
                .finish()
        };
        format!("instance/service-accounts/{}/token?{}", self.account, query)
    }

    pub async fn detect_google_project_id(&self) -> Option<String> {
        self.gcemeta_client.project_id().await.ok()
    }

    pub async fn id_token(&self, audience: &str) -> crate::error::Result<SecretValue> {
        let url = PathAndQuery::from_str(
            format!(
                "/computeMetadata/v1/instance/service-accounts/{}/identity?audience={}",
                self.account, audience
            )
            .as_str(),
        )?;
        trace!(
            "Receiving a new ID token from Metadata Server using '{}'",
            url
        );
        let resp = self.gcemeta_client.get(url, false).await?;
        Ok(SecretValue::from(resp))
    }
}

impl From<Metadata> for BoxSource {
    fn from(v: Metadata) -> Self {
        Box::new(v)
    }
}

#[async_trait]
impl Source for Metadata {
    async fn token(&self) -> crate::error::Result<Token> {
        let url =
            PathAndQuery::from_str(format!("/computeMetadata/v1/{}", self.uri_suffix()).as_str())?;
        trace!("Receiving a new token from Metadata Server using '{}'", url);

        let resp_str = self.gcemeta_client.get(url, false).await?;
        let resp = TokenResponse::try_from(resp_str.as_str())?;
        Token::try_from(resp)
    }
}

pub async fn from_metadata(
    scopes: &[String],
    account: &str,
) -> crate::error::Result<Option<Metadata>> {
    let gcemeta_client = gcemeta::Client::new();

    if gcemeta_client.on_gce().await? {
        Ok(Some(Metadata::with_account(scopes, account.to_string())))
    } else {
        Ok(None)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_metadata_uri_suffix() {
        let m = Metadata::new(Vec::new());
        assert_eq!(m.uri_suffix(), "instance/service-accounts/default/token?");

        let m = Metadata::new(crate::GCP_DEFAULT_SCOPES.clone());

        assert_eq!(
            m.uri_suffix(),
            "instance/service-accounts/default/token?scopes=https%3A%2F%2Fwww.googleapis.com%2Fauth%2Fcloud-platform"
        );

        let m = Metadata::new(vec!["scope1".into(), "scope2".into()]);
        assert_eq!(
            m.uri_suffix(),
            "instance/service-accounts/default/token?scopes=scope1%2Cscope2",
        );
    }
}
