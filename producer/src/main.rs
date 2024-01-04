use aws_config::BehaviorVersion;
use aws_sdk_sqs::{config::Builder, Client};
use proto::example::v1::Greet;
use serde::Deserialize;

#[derive(Clone, Deserialize)]
struct Config {
    #[serde(default = "String::new")]
    queue_url: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let app_config = envy::from_env::<Config>()?;

    let aws_config = aws_config::load_defaults(BehaviorVersion::latest()).await;
    let sqs_config_builder = Builder::from(&aws_config).endpoint_url("http://localhost:4566");
    let sqs_client = Client::from_conf(sqs_config_builder.build());

    let greet = Greet {
        message: "Hello, world!".into(),
    };

    let message = serde_json::to_string(&greet).unwrap();

    let _ = sqs_client
        .send_message()
        .queue_url(app_config.queue_url)
        .message_body(message)
        .send()
        .await
        .unwrap();

    println!("Message sent!");

    Ok(())
}
