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
            ..Default::default()
        }
    ).await?;

    println!("{:?}", response);


    Ok(())
}
