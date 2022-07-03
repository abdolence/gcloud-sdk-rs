use std::convert::TryFrom;
use gcloud_sdk::{
    google::spanner::admin::database::v1::{
        database_admin_client::DatabaseAdminClient, ListDatabasesRequest,
    },
    CERTIFICATES,
};
use gouth::Token;
use tonic::{
    metadata::MetadataValue,
    transport::{Certificate, Channel, ClientTlsConfig},
    Request,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let project = std::env::var("PROJECT")?;
    let instance = std::env::var("INSTANCE")?;
    let token = Token::new()?;

    let tls_config = ClientTlsConfig::new()
        .ca_certificate(Certificate::from_pem(CERTIFICATES))
        .domain_name("spanner.googleapis.com");

    let channel = Channel::from_static("https://spanner.googleapis.com")
        .tls_config(tls_config)?
        .connect()
        .await?;

    let mut service = DatabaseAdminClient::with_interceptor(channel, move |mut req: Request<()>| {
        let token = &*token.header_value().unwrap();
        let meta: MetadataValue<_> = MetadataValue::try_from(token).unwrap();
        req.metadata_mut().insert("authorization", meta);
        Ok(req)
    });

    let response = service
        .list_databases(Request::new(ListDatabasesRequest {
            parent: format!("projects/{}/instances/{}", project, instance),
            page_size: 100,
            ..Default::default()
        }))
        .await?;

    println!("RESPONSE={:?}", response);

    Ok(())
}
