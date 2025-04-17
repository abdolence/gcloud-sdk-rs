use gcloud_sdk::{
    error::Error, google_rest_apis::cloudresourcemanager_v3, GoogleEnvironment, GoogleRestApi,
};

#[tokio::main]
async fn main() -> Result<(), Error> {
    let client = GoogleRestApi::new().await?;
    let cfg = client
        .create_google_cloudresourcemanager_v3_config()
        .await?;

    let project = GoogleEnvironment::detect_google_project_id().await.unwrap();

    println!("{:#?}", cloudresourcemanager_v3::projects_api::cloudresourcemanager_projects_search(&cfg,
        cloudresourcemanager_v3::projects_api::CloudresourcemanagerPeriodProjectsPeriodSearchParams {
        query: Some(format!("name:{project}")),
        ..Default::default()
    }).await);

    Ok(())
}
