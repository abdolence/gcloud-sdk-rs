use gcloud_sdk::{google_rest_apis::servicecontrol_v1, GoogleEnvironment, GoogleRestApi};

#[derive(thiserror::Error, Debug)]
enum Error {
    #[error(transparent)]
    Gcloud(#[from] gcloud_sdk::error::Error),
    #[error(transparent)]
    ServiceCheck(
        #[from]
        servicecontrol_v1::Error<
            servicecontrol_v1::services_api::ServicecontrolPeriodServicesPeriodCheckError,
        >,
    ),
    #[error("{}", format_check_errors(.0))]
    ServiceCheckErrors(Vec<servicecontrol_v1::CheckError>),
}

fn format_check_errors(errs: &[servicecontrol_v1::CheckError]) -> String {
    errs.iter()
        .filter_map(|e| {
            if let Some(ref code) = e.code {
                let c = serde_json::to_string(code)
                    .unwrap_or_else(|_| "ERROR_CODE_UNSPECIFIED".to_string());
                if let Some(ref detail) = e.detail {
                    Some(format!("{c}: {detail}"))
                } else {
                    Some(c)
                }
            } else {
                None
            }
        })
        .collect::<Vec<_>>()
        .join(", ")
}

async fn services_check(
    client: GoogleRestApi,
    service_name: impl ToString,
) -> Result<(), Error> {
    let cfg = client.create_google_servicecontrol_v1_config().await?;

    let response = servicecontrol_v1::services_api::servicecontrol_services_check(
        &cfg,
        servicecontrol_v1::services_api::ServicecontrolPeriodServicesPeriodCheckParams {
            service_name: service_name.to_string(),
            check_request: Some(servicecontrol_v1::CheckRequest {
                operation: Some(Box::new(servicecontrol_v1::Operation {
                    start_time: Some(chrono::Utc::now().to_rfc3339()),
                    operation_id: Some(uuid::Uuid::new_v4().to_string()),
                    operation_name: Some("Whatever".to_string()),
                    consumer_id: GoogleEnvironment::detect_google_project_id()
                        .await
                        .map(|id| format!("project:{id}")),
                    ..Default::default()
                })),
                ..Default::default()
            }),
            ..Default::default()
        },
    )
    .await?;

    if let Some(errs) = response.check_errors {
        Err(Error::ServiceCheckErrors(errs))
    } else {
        Ok(())
    }
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let client = GoogleRestApi::new().await?;
    services_check(client, "sandbox-lustre.sandbox.googleapis.com").await?;

    Ok(())
}
