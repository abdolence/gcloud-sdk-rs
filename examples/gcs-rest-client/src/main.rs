#![allow(dead_code)]

use gcloud_sdk::GoogleRestApi;
use gcloud_sdk::{
    google::cloud::config,
    google_rest_apis::storage_v1::{
        configuration, BucketIamConfiguration, BucketIamConfigurationUniformBucketLevelAccess,
    },
};

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
            project: google_project_id.clone(),
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

    println!("========================================================");
    test_bucket_creation(
        "europe-west4".to_string(),
        google_project_id,
        &google_rest_client,
    )
    .await;
    println!("========================================================");

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

    let response = gcloud_sdk::google_rest_apis::compute_v1::instances_api::compute_instances_list(
        &google_rest_client.create_google_compute_v1_config().await.unwrap(),
        gcloud_sdk::google_rest_apis::compute_v1::instances_api::ComputePeriodInstancesPeriodListParams {
            project: google_project_id.to_string(),
            zone: "us-central1-a".to_string(),
            ..Default::default()
        }
    ).await.unwrap();

    println!("{:?}", response);
}

async fn test_bucket_creation(
    region: String,
    project_id: String,
    google_rest_client: &GoogleRestApi,
) {
    let bucket_name = format!("test-gcloud-sdk-rs-{}", uuid::Uuid::new_v4());
    println!("Creating bucket: {}", bucket_name);
    let insert_param = gcloud_sdk::google_rest_apis::storage_v1::buckets_api::StoragePeriodBucketsPeriodInsertParams {
        project: project_id,
        bucket: Some(gcloud_sdk::google_rest_apis::storage_v1::Bucket {
            name: Some( bucket_name.clone() ),
            location: Some(region),
            storage_class: Some("STANDARD".to_string()),
            retention_policy: None,
            lifecycle: None,
            iam_configuration: Some(Box::new(BucketIamConfiguration {
                uniform_bucket_level_access: Some(Box::new(BucketIamConfigurationUniformBucketLevelAccess {
                    enabled: Some(true),
                    locked_time: None,
                    ..Default::default()
                })),
                public_access_prevention: Some("enforced".to_string()),
                ..Default::default()
            })),
            soft_delete_policy: Some(Box::new(gcloud_sdk::google_rest_apis::storage_v1::bucket_soft_delete_policy::BucketSoftDeletePolicy{
                retention_duration_seconds: Some("0".to_string()),
                ..Default::default()
            })),
            ..Default::default()
        }),
        ..Default::default()
    };

    let configuration_result = google_rest_client.create_google_storage_v1_config().await;

    match configuration_result {
        Ok(config) => {
            let response =
                gcloud_sdk::google_rest_apis::storage_v1::buckets_api::storage_buckets_insert(
                    &config,
                    insert_param,
                )
                .await;

            match response {
                Ok(bucket) => {
                    println!("Bucket created: {}", bucket.name.unwrap());
                }
                Err(e) => {
                    println!("Error creating bucket: {:?}", e);
                }
            }
        }
        Err(e) => {
            println!("Error creating configuration: {:?}", e);
        }
    };
}
