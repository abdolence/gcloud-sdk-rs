use url::form_urlencoded::Serializer;

use async_trait::async_trait;
use hyper::http::uri::PathAndQuery;
use std::convert::TryFrom;
use std::str::FromStr;

use crate::source::{BoxSource, Source, Token, TokenResponse};

#[derive(Debug)]
pub struct Metadata {
    account: &'static str,
    scopes: Vec<String>,
    gcemeta_client: gcemeta::Client<hyper::client::connect::HttpConnector, hyper::Body>,
}

impl Metadata {
    fn new(scopes: impl Into<Vec<String>>) -> Self {
        Self::with_account(scopes, "default")
    }

    fn with_account(scopes: impl Into<Vec<String>>, account: &'static str) -> Self {
        Self {
            account: account,
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
}

impl From<Metadata> for BoxSource {
    fn from(v: Metadata) -> Self {
        Box::new(v)
    }
}

#[async_trait]
impl Source for Metadata {
    async fn token(&self) -> crate::Result<Token> {
        if !self.gcemeta_client.on_gce().await? {
            panic!("must be running on Google Compute Engine.")
        }
        let resp_str = self
            .gcemeta_client
            .get(PathAndQuery::from_str(&self.uri_suffix())?, false)
            .await?;
        let resp = TokenResponse::try_from(resp_str.as_str())?;
        Token::try_from(resp)
    }
}

pub async fn from_metadata(scopes: &[String]) -> crate::Result<Option<Metadata>> {
    let gcemeta_client = gcemeta::Client::new();

    if gcemeta_client.on_gce().await? {
        Ok(Some(Metadata::new(scopes)))
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

        let m = Metadata::new(vec!["https://www.googleapis.com/auth/cloud-platform".into()]);
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
