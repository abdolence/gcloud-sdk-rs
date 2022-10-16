use crate::token_source::credentials::ExternalAccount;
use secret_vault_value::SecretValue;
use std::collections::HashMap;

use tracing::*;

#[derive(Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(untagged)]
pub enum ExternalCredentialSource {
    UrlBased(ExternalCredentialUrl),
    FileBased(ExternalCredentialFile),
    // AWS external source implementation example https://github.com/googleapis/google-auth-library-nodejs/blob/4bbd13fbf9081e004209d0ffc336648cff0c529e/src/auth/awsclient.ts
}

#[derive(Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct ExternalCredentialUrl {
    url: String,
    headers: Option<HashMap<String, SecretValue>>,
    format: Option<ExternalCredentialUrlFormat>,
}

#[derive(Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct ExternalCredentialFile {
    file: String,
    format: Option<ExternalCredentialUrlFormat>,
}

#[derive(Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(tag = "type")]
#[serde(rename_all = "camelCase")]
pub enum ExternalCredentialUrlFormat {
    Json(ExternalCredentialUrlFormatJson),
    Text,
}

#[derive(Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct ExternalCredentialUrlFormatJson {
    pub subject_token_field_name: String,
}

pub async fn subject_token(
    client: &reqwest::Client,
    external_account: &ExternalAccount,
) -> crate::error::Result<SecretValue> {
    match external_account.credential_source {
        ExternalCredentialSource::UrlBased(ref url_creds) => {
            subject_token_url(client, url_creds).await
        }
        ExternalCredentialSource::FileBased(ref url_creds) => subject_token_file(url_creds).await,
    }
}

pub async fn subject_token_url(
    client: &reqwest::Client,
    url_creds: &ExternalCredentialUrl,
) -> crate::error::Result<SecretValue> {
    debug!(
        "Using external credentials URL source {}. Format: {:?}",
        &url_creds.url, &url_creds.format
    );
    let mut request = client.get(url_creds.url.as_str());

    if let Some(headers) = &url_creds.headers {
        for (header_name, header_value) in headers {
            request = request.header(header_name, header_value.as_sensitive_str());
        }
    }

    let response = request.send().await?;

    if response.status().is_success() {
        if let Some(format) = &url_creds.format {
            match format {
                ExternalCredentialUrlFormat::Json(json_settings) => {
                    let json: serde_json::Value = response.json().await?;
                    let json_object = json.as_object().ok_or_else(|| {
                        crate::error::ErrorKind::ExternalCredsSourceError(format!(
                            "External subject JSON format is not object: {}",
                            json
                        ))
                    })?;

                    if let Some(subject_json_value) =
                        json_object.get(&json_settings.subject_token_field_name)
                    {
                        if let Some(subject_json_value_str) = subject_json_value.as_str() {
                            Ok(subject_json_value_str.into())
                        } else {
                            Err(crate::error::ErrorKind::ExternalCredsSourceError(format!(
                                "External subject JSON field must have string type: {}",
                                &json_settings.subject_token_field_name
                            ))
                            .into())
                        }
                    } else {
                        Err(crate::error::ErrorKind::ExternalCredsSourceError(format!(
                            "External subject JSON format doesn't contain required field: {}",
                            &json_settings.subject_token_field_name
                        ))
                        .into())
                    }
                }
                ExternalCredentialUrlFormat::Text => Ok(response.text().await?.into()),
            }
        } else {
            Ok(response.text().await?.into())
        }
    } else {
        let status = response.status();
        let err_body = response.text().await?;
        let err_text = format!(
            "Unable to receive subject using external credential url: {}. HTTP: {} {}",
            &url_creds.url, status, err_body
        );
        Err(crate::error::ErrorKind::ExternalCredsSourceError(err_text).into())
    }
}

pub async fn subject_token_file(
    url_creds: &ExternalCredentialFile,
) -> crate::error::Result<SecretValue> {
    debug!(
        "Using external credentials file source {}. Format: {:?}",
        &url_creds.file, &url_creds.format
    );
    let file_content: String = std::fs::read_to_string(url_creds.file.as_str()).map_err(|e| {
        crate::error::ErrorKind::ExternalCredsSourceError(format!(
            "External file is not readable: {}",
            e
        ))
    })?;

    if let Some(format) = &url_creds.format {
        match format {
            ExternalCredentialUrlFormat::Json(json_settings) => {
                let json: serde_json::Value =
                    serde_json::from_str(file_content.as_str()).map_err(|e| {
                        crate::error::ErrorKind::ExternalCredsSourceError(format!(
                            "External file JSON format error: {}",
                            e
                        ))
                    })?;
                let json_object = json.as_object().ok_or_else(|| {
                    crate::error::ErrorKind::ExternalCredsSourceError(format!(
                        "External subject JSON format is not object: {}",
                        json
                    ))
                })?;

                if let Some(subject_json_value) =
                    json_object.get(&json_settings.subject_token_field_name)
                {
                    if let Some(subject_json_value_str) = subject_json_value.as_str() {
                        Ok(subject_json_value_str.into())
                    } else {
                        Err(crate::error::ErrorKind::ExternalCredsSourceError(format!(
                            "External subject JSON field must have string type: {}",
                            &json_settings.subject_token_field_name
                        ))
                        .into())
                    }
                } else {
                    Err(crate::error::ErrorKind::ExternalCredsSourceError(format!(
                        "External subject JSON format doesn't contain required field: {}",
                        &json_settings.subject_token_field_name
                    ))
                    .into())
                }
            }
            ExternalCredentialUrlFormat::Text => Ok(file_content.into()),
        }
    } else {
        Ok(file_content.into())
    }
}
