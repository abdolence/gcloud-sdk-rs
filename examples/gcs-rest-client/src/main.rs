#![allow(dead_code)]

use gcloud_sdk::GoogleRestApi;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Debug logging
    let subscriber = tracing_subscriber::fmt()
        .with_env_filter("gcloud_sdk=debug")
        .finish();
    tracing::subscriber::set_global_default(subscriber)?;

    // Detect Google project ID using environment variables PROJECT_ID/GCP_PROJECT_ID
    // or GKE metadata server when the app runs inside GKE
    let google_project_id = gcloud_sdk::GoogleEnvironment::detect_google_project_id().await
        .expect("No Google Project ID detected. Please specify it explicitly using env variable: PROJECT_ID");

    let google_rest_client = gcloud_sdk::GoogleRestApi::new().await?;

    let response = gcloud_sdk::google_rest_apis::storage_v1::buckets_api::storage_buckets_list(
        &google_rest_client.create_google_storage_v1_config().await?,
        gcloud_sdk::google_rest_apis::storage_v1::buckets_api::StoragePeriodBucketsPeriodListParams {
            project: google_project_id,
            alt: None,
            fields: None,
            key: None,
            oauth_token: None,
            pretty_print: None,
            quota_user: None,
            upload_type: None,
            user_ip: None,
            max_results: None,
            page_token: None,
            prefix: None,
            projection: None,
            user_project: None,
        }
    ).await?;

    println!("{:?}", response);

    test_compute().await;

    Ok(())
}

// Upload to GCS has a slightly different API that OpenAPI doesn't support, so there is an extension method in this library to support this
async fn test_upload(
    bucket: &str,
    filename: &str,
    google_rest_client: &GoogleRestApi,
) -> Result<(), Box<dyn std::error::Error>> {
    let response = gcloud_sdk::google_rest_apis::storage_v1::objects_api::storage_objects_insert_ext_bytes(
        &google_rest_client.create_google_storage_v1_config().await?,
        gcloud_sdk::google_rest_apis::storage_v1::objects_api::StoragePeriodObjectsPeriodInsertParams {
            bucket: bucket.to_string(),
            name: Some(filename.to_string()),
            ..Default::default()
        },
        None,
        "Hello".as_bytes().to_vec()
    ).await?;

    println!("{:?}", response);

    Ok(())
}

async fn test_compute() {
    let google_project_id = gcloud_sdk::GoogleEnvironment::detect_google_project_id().await
        .expect("No Google Project ID detected. Please specify it explicitly using env variable: PROJECT_ID");

    let google_rest_client = gcloud_sdk::GoogleRestApi::new().await.unwrap();

    let compute_config = google_rest_client
        .create_google_compute_v1_config()
        .await
        .unwrap();
    let request = gcloud_sdk::google_rest_apis::compute_v1::instances_api::ComputePeriodInstancesPeriodListParams {
        project: google_project_id.to_string(),
        zone: "us-central1-a".into(),
        ..Default::default()
    };
    let response = gcloud_sdk::google_rest_apis::compute_v1::instances_api::compute_instances_list(
        &compute_config,
        request,
    )
    .await
    .unwrap();

    println!("{:?}", response);
}
