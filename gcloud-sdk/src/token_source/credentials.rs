use std::{
    env, fs,
    path::{Path, PathBuf},
};

use async_trait::async_trait;

use crate::token_source::{BoxSource, Source, Token};

#[derive(Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(untagged)]
pub enum Credentials {
    ServiceAccount(ServiceAccount),
    User(User),
}

#[derive(Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct ServiceAccount {
    client_email: String,
    private_key_id: String,
    private_key: String,
    token_uri: String,
    #[serde(skip)]
    scopes: Vec<String>,
}

#[derive(Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct User {
    client_secret: String,
    client_id: String,
    refresh_token: String,
}

#[async_trait]
impl Source for Credentials {
    async fn token(&self) -> crate::error::Result<Token> {
        use Credentials::*;
        match self {
            ServiceAccount(sa) => jwt::token(sa),
            User(user) => oauth2::token(user),
        }
    }
}

impl From<Credentials> for BoxSource {
    fn from(v: Credentials) -> Self {
        Box::new(v)
    }
}

const ENV_KEY: &str = "GOOGLE_APPLICATION_CREDENTIALS";

pub fn from_env_var(scopes: &[String]) -> crate::error::Result<Option<Credentials>> {
    match env::var(ENV_KEY) {
        Ok(path) => from_file(path, scopes).map(Some),
        Err(_) => Ok(None),
    }
}

pub fn from_well_known_file(scopes: &[String]) -> crate::error::Result<Option<Credentials>> {
    match well_known_file_path() {
        Some(path) if path.exists() => from_file(path, scopes).map(Some),
        _ => Ok(None),
    }
}

fn well_known_file_path() -> Option<PathBuf> {
    let mut buf = {
        #[cfg(target_os = "windows")]
        {
            PathBuf::from(env::var("APPDATA").ok()?)
        }
        #[cfg(not(target_os = "windows"))]
        {
            let mut buf = PathBuf::from(env::var("HOME").ok()?);
            buf.push(".config");
            buf
        }
    };
    buf.push("gcloud");
    buf.push("application_default_credentials.json");
    Some(buf)
}

pub fn from_json(buf: &[u8], scopes: &[String]) -> crate::error::Result<Credentials> {
    let mut creds =
        serde_json::from_slice(buf).map_err(crate::error::ErrorKind::CredentialsJson)?;
    if let Credentials::ServiceAccount(ref mut sa) = creds {
        sa.scopes = scopes.to_owned()
    }
    Ok(creds)
}

pub fn from_file(path: impl AsRef<Path>, scopes: &[String]) -> crate::error::Result<Credentials> {
    let buf = fs::read(path).map_err(crate::error::ErrorKind::CredentialsFile)?;
    from_json(&buf, scopes)
}

#[inline]
fn httpc_post(url: &str) -> attohttpc::RequestBuilder {
    attohttpc::post(url).header_append(attohttpc::header::USER_AGENT, crate::GCLOUD_SDK_USER_AGENT)
}

mod jwt {
    use jsonwebtoken::{encode, Algorithm, EncodingKey, Header};

    use std::{convert::TryFrom, time::SystemTime};

    use crate::token_source::{
        credentials::{httpc_post, ServiceAccount},
        Token, TokenResponse,
    };

    #[derive(Debug, serde::Serialize, serde::Deserialize)]
    struct Claims<'a> {
        iss: &'a str,
        scope: &'a str,
        aud: &'a str,
        iat: u64,
        exp: u64,
    }

    // If client machine's time is in the future according
    // to Google servers, an access token will not be issued.
    fn issued_at() -> u64 {
        SystemTime::UNIX_EPOCH.elapsed().unwrap().as_secs() - 10
    }

    // https://cloud.google.com/iot/docs/concepts/device-security#security_standards
    fn header(typ: &str, prv_key_id: &str) -> Header {
        Header {
            typ: Some(typ.into()),
            alg: Algorithm::RS256,
            kid: Some(prv_key_id.into()),
            ..Header::default()
        }
    }

    #[derive(Debug, serde::Serialize, serde::Deserialize)]
    struct Payload<'a> {
        grant_type: &'a str,
        assertion: &'a str,
    }

    const DEFAULT_EXPIRE: u64 = 60 * 60;

    pub fn token(sa: &ServiceAccount) -> crate::error::Result<Token> {
        let iat = issued_at();
        let claims = Claims {
            iss: &sa.client_email,
            scope: &sa.scopes.join(" "),
            aud: &sa.token_uri,
            iat,
            exp: iat + DEFAULT_EXPIRE,
        };
        let header = header("JWT", &sa.private_key_id);
        let key = EncodingKey::from_rsa_pem(sa.private_key.as_bytes())?;
        let assertion = &encode(&header, &claims, &key)?;

        let mut req = httpc_post(&sa.token_uri)
            .form(&Payload {
                grant_type: "urn:ietf:params:oauth:grant-type:jwt-bearer",
                assertion,
            })?
            .prepare();
        let resp = req.send()?;
        if resp.is_success() {
            let resp = TokenResponse::try_from(resp.text()?.as_ref())?;
            Token::try_from(resp)
        } else {
            Err(crate::error::ErrorKind::HttpStatus(resp.status()).into())
        }
    }
}

mod oauth2 {
    use std::convert::TryFrom;

    use crate::token_source::{
        credentials::{httpc_post, User},
        Token, TokenResponse,
    };

    #[derive(serde::Serialize)]
    struct Payload<'a> {
        client_id: &'a str,
        client_secret: &'a str,
        grant_type: &'a str,
        refresh_token: &'a str,
    }

    // https://github.com/golang/oauth2/blob/bf48bf16ab8d622ce64ec6ce98d2c98f916b6303/google/google.go#L21-L26
    const TOKEN_URL: &str = "https://oauth2.googleapis.com/token";
    const GRANT_TYPE: &str = "refresh_token";

    pub fn token(user: &User) -> crate::error::Result<Token> {
        fetch_token(TOKEN_URL, user)
    }

    pub(super) fn fetch_token(url: &str, user: &User) -> crate::error::Result<Token> {
        let mut req = httpc_post(url)
            .form(&Payload {
                client_id: &user.client_id,
                client_secret: &user.client_secret,
                grant_type: GRANT_TYPE,
                // The reflesh token is not included in the response from google's server,
                // so it always uses the specified refresh token from the file.
                refresh_token: &user.refresh_token,
            })?
            .prepare();
        let resp = req.send()?;
        if resp.is_success() {
            let resp = TokenResponse::try_from(resp.text()?.as_ref())?;
            Token::try_from(resp)
        } else {
            Err(crate::error::ErrorKind::HttpStatus(resp.status()).into())
        }
    }
}
