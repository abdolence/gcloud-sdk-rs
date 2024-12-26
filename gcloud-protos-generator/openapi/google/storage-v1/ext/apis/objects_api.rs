pub type BoxStreamWithSync<'a, T> = std::pin::Pin<Box<dyn futures::Stream<Item = T> + Send + 'a + Sync>>;
pub type BoxStreamWithSend<'a, T> = std::pin::Pin<Box<dyn futures::Stream<Item = T> + Send + 'a>>;

/// Stores a new object and metadata.
/// Open API doesn't support binary streams and this particular endpoint uses another base URL.
/// Tha means generated `storage_objects_insert` can't be used.
pub async fn storage_objects_insert_ext_stream<S>(
    configuration: &configuration::Configuration,
    params: StoragePeriodObjectsPeriodInsertParams,
    content_type: Option<String>,
    bytes_stream: S,
) -> Result<
    crate::google_rest_apis::storage_v1::models::Object,
    Error<StoragePeriodObjectsPeriodInsertError>,
>
where
    S: futures::stream::TryStream + Send + 'static,
    S::Error: Into<Box<dyn std::error::Error + Send + Sync>>,
    bytes::Bytes: From<S::Ok>,
{
    let local_var_configuration = configuration;

    // unbox the parameters
    let bucket = params.bucket;
    let alt = params.alt;
    let fields = params.fields;
    let key = params.key;
    let oauth_token = params.oauth_token;
    let pretty_print = params.pretty_print;
    let quota_user = params.quota_user;
    let upload_type = "media";
    let user_ip = params.user_ip;
    let content_encoding = params.content_encoding;
    let if_generation_match = params.if_generation_match;
    let if_generation_not_match = params.if_generation_not_match;
    let if_metageneration_match = params.if_metageneration_match;
    let if_metageneration_not_match = params.if_metageneration_not_match;
    let kms_key_name = params.kms_key_name;
    let name = params.name;
    let predefined_acl = params.predefined_acl;
    let projection = params.projection;
    let user_project = params.user_project;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "https://storage.googleapis.com/upload/storage/v1/b/{bucket}/o",
        bucket = crate::google_rest_apis::storage_v1::apis::urlencode(bucket)
    );
    let mut local_var_req_builder =
        local_var_client
            .request(reqwest::Method::POST, local_var_uri_str.as_str());

    if let Some(content_type) = content_type {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::CONTENT_TYPE, content_type);
    }
    if let Some(ref local_var_str) = alt {
        local_var_req_builder = local_var_req_builder.query(&[("alt", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = fields {
        local_var_req_builder =
            local_var_req_builder.query(&[("fields", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = key {
        local_var_req_builder = local_var_req_builder.query(&[("key", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = oauth_token {
        local_var_req_builder =
            local_var_req_builder.query(&[("oauth_token", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = pretty_print {
        local_var_req_builder =
            local_var_req_builder.query(&[("prettyPrint", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = quota_user {
        local_var_req_builder =
            local_var_req_builder.query(&[("quotaUser", &local_var_str.to_string())]);
    }

    local_var_req_builder =
        local_var_req_builder.query(&[("uploadType", &upload_type)]);

    if let Some(ref local_var_str) = user_ip {
        local_var_req_builder =
            local_var_req_builder.query(&[("userIp", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = content_encoding {
        local_var_req_builder =
            local_var_req_builder.query(&[("contentEncoding", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = if_generation_match {
        local_var_req_builder =
            local_var_req_builder.query(&[("ifGenerationMatch", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = if_generation_not_match {
        local_var_req_builder =
            local_var_req_builder.query(&[("ifGenerationNotMatch", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = if_metageneration_match {
        local_var_req_builder =
            local_var_req_builder.query(&[("ifMetagenerationMatch", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = if_metageneration_not_match {
        local_var_req_builder = local_var_req_builder
            .query(&[("ifMetagenerationNotMatch", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = kms_key_name {
        local_var_req_builder =
            local_var_req_builder.query(&[("kmsKeyName", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = name {
        local_var_req_builder =
            local_var_req_builder.query(&[("name", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = predefined_acl {
        local_var_req_builder =
            local_var_req_builder.query(&[("predefinedAcl", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = projection {
        local_var_req_builder =
            local_var_req_builder.query(&[("projection", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = user_project {
        local_var_req_builder =
            local_var_req_builder.query(&[("userProject", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder =
            local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_token) = local_var_configuration.oauth_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };
    if let Some(ref local_var_token) = local_var_configuration.oauth_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };
    use reqwest::Body;
    local_var_req_builder = local_var_req_builder.body(Body::wrap_stream(bytes_stream));

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<StoragePeriodObjectsPeriodInsertError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}


pub async fn storage_objects_insert_ext_bytes(
    configuration: &configuration::Configuration,
    params: StoragePeriodObjectsPeriodInsertParams,
    content_type: Option<String>,
    bytes: Vec<u8>,
) -> Result<
    crate::google_rest_apis::storage_v1::models::Object,
    Error<StoragePeriodObjectsPeriodInsertError>,
> {
    use futures::StreamExt;

    let bytes_stream: BoxStreamWithSend<
        'static,
        std::result::Result<bytes::Bytes, Box<(dyn std::error::Error + Send + Sync + 'static)>>,
    > = Box::pin(
        futures::stream::iter(
            bytes
        ).chunks(64 * 1024).map(|chunk| Ok(bytes::Bytes::from(chunk)))
    );

    storage_objects_insert_ext_stream(configuration, params, content_type, bytes_stream).await
}

/// Retrieves the contents of an object as bytes
pub async fn storage_objects_get_bytes(
    configuration: &configuration::Configuration,
    params: StoragePeriodObjectsPeriodGetParams) -> Result<bytes::Bytes,Error<StoragePeriodObjectsPeriodGetError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let bucket = params.bucket;
    let object = params.object;
    let fields = params.fields;
    let key = params.key;
    let oauth_token = params.oauth_token;
    let pretty_print = params.pretty_print;
    let quota_user = params.quota_user;
    let upload_type = params.upload_type;
    let user_ip = params.user_ip;
    let generation = params.generation;
    let if_generation_match = params.if_generation_match;
    let if_generation_not_match = params.if_generation_not_match;
    let if_metageneration_match = params.if_metageneration_match;
    let if_metageneration_not_match = params.if_metageneration_not_match;
    let projection = params.projection;
    let user_project = params.user_project;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/b/{bucket}/o/{object}",
        local_var_configuration.base_path,
        bucket = crate::google_rest_apis::storage_v1::apis::urlencode(bucket),
        object = crate::google_rest_apis::storage_v1::apis::urlencode(object)
    );
    let mut local_var_req_builder =
        local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    // The user-supplied `alt` parameter is ignored, because it must be "media"
    local_var_req_builder = local_var_req_builder.query(&[("alt", "media".to_string())]);

    if let Some(ref local_var_str) = fields {
        local_var_req_builder =
            local_var_req_builder.query(&[("fields", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = key {
        local_var_req_builder = local_var_req_builder.query(&[("key", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = oauth_token {
        local_var_req_builder =
            local_var_req_builder.query(&[("oauth_token", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = pretty_print {
        local_var_req_builder =
            local_var_req_builder.query(&[("prettyPrint", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = quota_user {
        local_var_req_builder =
            local_var_req_builder.query(&[("quotaUser", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = upload_type {
        local_var_req_builder =
            local_var_req_builder.query(&[("uploadType", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = user_ip {
        local_var_req_builder =
            local_var_req_builder.query(&[("userIp", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = generation {
        local_var_req_builder =
            local_var_req_builder.query(&[("generation", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = if_generation_match {
        local_var_req_builder =
            local_var_req_builder.query(&[("ifGenerationMatch", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = if_generation_not_match {
        local_var_req_builder =
            local_var_req_builder.query(&[("ifGenerationNotMatch", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = if_metageneration_match {
        local_var_req_builder =
            local_var_req_builder.query(&[("ifMetagenerationMatch", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = if_metageneration_not_match {
        local_var_req_builder = local_var_req_builder
            .query(&[("ifMetagenerationNotMatch", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = projection {
        local_var_req_builder =
            local_var_req_builder.query(&[("projection", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = user_project {
        local_var_req_builder =
            local_var_req_builder.query(&[("userProject", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder =
            local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_token) = local_var_configuration.oauth_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };
    if let Some(ref local_var_token) = local_var_configuration.oauth_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        Ok(local_var_resp.bytes().await?)
    } else {
        let local_var_content = local_var_resp.text().await?;
        let local_var_entity: Option<StoragePeriodObjectsPeriodGetError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Retrieves the contents of an object as stream
pub async fn storage_objects_get_stream(
    configuration: &configuration::Configuration,
    params: StoragePeriodObjectsPeriodGetParams) -> Result<impl futures::stream::Stream<Item = std::result::Result<bytes::Bytes, reqwest::Error>>,Error<StoragePeriodObjectsPeriodGetError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let bucket = params.bucket;
    let object = params.object;
    let fields = params.fields;
    let key = params.key;
    let oauth_token = params.oauth_token;
    let pretty_print = params.pretty_print;
    let quota_user = params.quota_user;
    let upload_type = params.upload_type;
    let user_ip = params.user_ip;
    let generation = params.generation;
    let if_generation_match = params.if_generation_match;
    let if_generation_not_match = params.if_generation_not_match;
    let if_metageneration_match = params.if_metageneration_match;
    let if_metageneration_not_match = params.if_metageneration_not_match;
    let projection = params.projection;
    let user_project = params.user_project;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/b/{bucket}/o/{object}",
        local_var_configuration.base_path,
        bucket = crate::google_rest_apis::storage_v1::apis::urlencode(bucket),
        object = crate::google_rest_apis::storage_v1::apis::urlencode(object)
    );
    let mut local_var_req_builder =
        local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    // The user-supplied `alt` parameter is ignored, because it must be "media"
    local_var_req_builder = local_var_req_builder.query(&[("alt", "media".to_string())]);

    if let Some(ref local_var_str) = fields {
        local_var_req_builder =
            local_var_req_builder.query(&[("fields", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = key {
        local_var_req_builder = local_var_req_builder.query(&[("key", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = oauth_token {
        local_var_req_builder =
            local_var_req_builder.query(&[("oauth_token", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = pretty_print {
        local_var_req_builder =
            local_var_req_builder.query(&[("prettyPrint", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = quota_user {
        local_var_req_builder =
            local_var_req_builder.query(&[("quotaUser", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = upload_type {
        local_var_req_builder =
            local_var_req_builder.query(&[("uploadType", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = user_ip {
        local_var_req_builder =
            local_var_req_builder.query(&[("userIp", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = generation {
        local_var_req_builder =
            local_var_req_builder.query(&[("generation", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = if_generation_match {
        local_var_req_builder =
            local_var_req_builder.query(&[("ifGenerationMatch", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = if_generation_not_match {
        local_var_req_builder =
            local_var_req_builder.query(&[("ifGenerationNotMatch", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = if_metageneration_match {
        local_var_req_builder =
            local_var_req_builder.query(&[("ifMetagenerationMatch", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = if_metageneration_not_match {
        local_var_req_builder = local_var_req_builder
            .query(&[("ifMetagenerationNotMatch", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = projection {
        local_var_req_builder =
            local_var_req_builder.query(&[("projection", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = user_project {
        local_var_req_builder =
            local_var_req_builder.query(&[("userProject", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder =
            local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_token) = local_var_configuration.oauth_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };
    if let Some(ref local_var_token) = local_var_configuration.oauth_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        Ok(local_var_resp.bytes_stream())
    } else {
        let local_var_content = local_var_resp.text().await?;
        let local_var_entity: Option<StoragePeriodObjectsPeriodGetError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}
