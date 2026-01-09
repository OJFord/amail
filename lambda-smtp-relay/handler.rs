#[macro_use]
extern crate log;

use std::env;
use std::str::FromStr;

use aws_lambda_events::event::ses::SimpleEmailEvent;
use lambda_runtime::Error;
use lambda_runtime::LambdaEvent;
use lambda_runtime::service_fn;
use lettre::Transport;
use lettre::transport::smtp;
use rusoto_core::Region;
use rusoto_s3::GetObjectRequest;
use rusoto_s3::S3;
use rusoto_s3::S3Client;
use serde_derive::Serialize;
use tokio::io::AsyncReadExt;

#[derive(Serialize)]
struct Output {}

#[tokio::main]
async fn main() -> Result<(), Error> {
    stderrlog::new()
        .modules(vec![module_path!(), "lettre"])
        .verbosity(2)
        .timestamp(stderrlog::Timestamp::Off)
        .init()
        .unwrap();

    lambda_runtime::run(service_fn(handler)).await
}

async fn handler(e: LambdaEvent<SimpleEmailEvent>) -> Result<Output, Error> {
    let region = env::var("S3_REGION")
        .expect("Missing $S3_REGION")
        .parse::<Region>()
        .expect("Invalid S3 region");
    let s3 = S3Client::new(region);

    let smtp_host = env::var("SMTP_HOST").expect("Missing $SMTP_HOST");
    let mut smtp = smtp::SmtpTransport::relay(&smtp_host)
        .expect("Failed to create SMTP client")
        .credentials(smtp::authentication::Credentials::new(
            env::var("SMTP_USER").expect("Missing $SMTP_USER"),
            env::var("SMTP_PASS").expect("Missing $SMTP_PASS"),
        ))
        .build();

    let mail_event = &e.payload.records[0].ses.mail;
    let message_id = mail_event
        .message_id
        .as_ref()
        .expect("Unknown SES messageId");

    info!("Relaying {}", message_id);

    relay_eml(&mut smtp, &s3, message_id).await;
    Ok(Output {})
}

async fn relay_eml(smtp: &mut smtp::SmtpTransport, s3: &S3Client, message_id: &str) {
    let mut content = Vec::new();
    s3.get_object(GetObjectRequest {
        bucket: env::var("S3_BUCKET").expect("Missing $S3_BUCKET"),
        key: message_id.into(),
        ..Default::default()
    })
    .await
    .unwrap_or_else(|_| panic!("Failed to retrieve {message_id}"))
    .body
    .unwrap()
    .into_async_read()
    .read_to_end(&mut content)
    .await
    .unwrap_or_else(|_| panic!("Failed to read {message_id}"));

    let relay_from = lettre::Address::from_str(
        &env::var("RELAY_ENVELOPE_FROM").expect("Missing $RELAY_ENVELOPE_FROM"),
    )
    .expect("Malformed $RELAY_ENVELOPE_FROM");

    let relay_to = lettre::Address::from_str(
        &env::var("RELAY_ENVELOPE_TO").expect("Missing $RELAY_ENVELOPE_TO"),
    )
    .expect("Malformed $RELAY_ENVELOPE_FROM");

    let envelope = lettre::address::Envelope::new(Some(relay_from), vec![relay_to])
        .expect("Failed to build envelope");

    smtp.send_raw(&envelope, &content)
        .unwrap_or_else(|_| panic!("Failed to send {message_id}"));
}
