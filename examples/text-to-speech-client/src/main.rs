use gcloud_sdk::*;
use gcloud_sdk::google::cloud::texttospeech::v1::text_to_speech_client::TextToSpeechClient;
use gcloud_sdk::google::cloud::texttospeech::v1::ListVoicesRequest;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Debug logging
    let subscriber = tracing_subscriber::fmt()
        .with_env_filter("gcloud_sdk=debug")
        .finish();
    tracing::subscriber::set_global_default(subscriber)?;

    let client: GoogleApi<TextToSpeechClient<GoogleAuthMiddleware>> =
        GoogleApi::from_function(
            TextToSpeechClient::new,
            "https://texttospeech.googleapis.com",
            // cloud resource prefix: used only for some of the APIs (such as Firestore)
            None,
        )
        .await?;

    let response = client
        .get()
        .list_voices(tonic::Request::new(ListVoicesRequest {
            language_code: "en-US".to_string(),
            ..Default::default()
        }))
        .await?;
    println!("Response: {:?}", response);

    Ok(())
}
