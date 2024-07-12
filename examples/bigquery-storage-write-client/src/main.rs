use futures::TryStreamExt;
use gcloud_sdk::google::cloud::bigquery::storage::v1::append_rows_request::ProtoData;
use gcloud_sdk::google::cloud::bigquery::storage::v1::big_query_write_client::BigQueryWriteClient;
use gcloud_sdk::google::cloud::bigquery::storage::v1::{
    append_rows_request, AppendRowsRequest, AppendRowsResponse, ProtoRows, ProtoSchema,
};

use crate::prost::*;
use crate::prost_types::*;
use gcloud_sdk::*;

#[derive(Clone, PartialEq, ::prost::Message)]
struct MySchemaRow {
    #[prost(string, tag = "1")]
    test_column: String,
    #[prost(int64, tag = "2")]
    test_num: i64,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Debug logging
    let subscriber = tracing_subscriber::fmt()
        .with_env_filter("gcloud_sdk=debug")
        .finish();
    tracing::subscriber::set_global_default(subscriber)?;

    // Detect Google project ID using environment variables PROJECT_ID/GCP_PROJECT_ID
    // or GKE metadata server when the app runs inside GKE
    let google_project_id = GoogleEnvironment::detect_google_project_id().await
        .expect("No Google Project ID detected. Please specify it explicitly using env variable: PROJECT_ID");

    let dataset = std::env::var("BQ_DATASET").expect("No BigQuery dataset is specified");
    let table = std::env::var("BQ_TABLE").expect("No BigQuery table is specified");

    let write_client: GoogleApi<BigQueryWriteClient<GoogleAuthMiddleware>> =
        GoogleApi::from_function(
            BigQueryWriteClient::new,
            "https://bigquerystorage.googleapis.com",
            None,
        )
        .await?;

    let rows_schema = ProtoSchema {
        proto_descriptor: Some(DescriptorProto {
            name: Some("test_schema".to_string()),
            field: vec![
                FieldDescriptorProto {
                    name: Some("test_column".to_string()),
                    number: Some(1),
                    r#type: Some(field_descriptor_proto::Type::String.into()),
                    ..Default::default()
                },
                FieldDescriptorProto {
                    name: Some("test_num".to_string()),
                    number: Some(2),
                    r#type: Some(field_descriptor_proto::Type::Int64.into()),
                    ..Default::default()
                },
            ],
            ..Default::default()
        }),
    };

    let rows_stream = futures::stream::iter(vec![AppendRowsRequest {
        write_stream: format!(
            "projects/{google_project_id}/datasets/{dataset}/tables/{table}/streams/_default"
        ),
        offset: None,
        trace_id: "test".to_string(),
        rows: Some(append_rows_request::Rows::ProtoRows(ProtoData {
            writer_schema: Some(rows_schema),
            rows: Some(ProtoRows {
                serialized_rows: vec![
                    MySchemaRow {
                        test_column: "test-1".to_string(),
                        test_num: 42,
                    }
                    .encode_to_vec(),
                    MySchemaRow {
                        test_column: "test-2".to_string(),
                        test_num: 42,
                    }
                    .encode_to_vec(),
                ],
            }),
        })),
        ..Default::default()
    }]);

    let response = write_client.get().append_rows(rows_stream).await?;

    let response_stream = response.into_inner();

    let collected: Vec<AppendRowsResponse> = response_stream.try_collect().await?;

    println!("Response: {:?}", collected);

    Ok(())
}
