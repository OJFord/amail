extern crate aws_lambda_events;
extern crate futures;
extern crate lambda_runtime;
extern crate lettre;
extern crate rusoto_core;
extern crate rusoto_s3;
extern crate rustls;
extern crate serde;
extern crate serde_derive;
extern crate tokio;
extern crate webpki_roots;

use futures::prelude::*;
use lettre::smtp;
use lettre::Transport;
use std::boxed::Box;
use std::env;
use std::error::Error;

use aws_lambda_events::event::ses::SimpleEmailEvent;
use lambda_runtime::error::HandlerError;
use lambda_runtime::lambda;
use lambda_runtime::Context;
use rusoto_core::Region;
use rusoto_s3::GetObjectRequest;
use rusoto_s3::S3Client;
use rusoto_s3::S3;
use serde_derive::Serialize;

#[derive(Serialize)]
struct Output {}

fn main() -> Result<(), Box<dyn Error>> {
    lambda!(handler);
    Ok(())
}

fn handler(e: SimpleEmailEvent, _: Context) -> Result<Output, HandlerError> {
    let region = env::var("S3_REGION")?.parse::<Region>().unwrap();
    let s3 = S3Client::new(region);

    let smtp_host = env::var("SMTP_HOST")?;
    let mut tls = rustls::ClientConfig::new();
    tls.root_store
        .add_server_trust_anchors(&webpki_roots::TLS_SERVER_ROOTS);
    let security = smtp::ClientSecurity::Wrapper(smtp::client::net::ClientTlsParameters::new(
        smtp_host.clone(),
        tls,
    ));
    let mut smtp = smtp::SmtpClient::new((smtp_host.as_ref(), 465 as u16), security)
        .unwrap()
        .credentials(smtp::authentication::Credentials::new(
            env::var("SMTP_USER").unwrap(),
            env::var("SMTP_PASS").unwrap(),
        ));

    let mail_event = &e.records[0].ses.mail;
    let message_id = mail_event
        .message_id
        .as_ref()
        .expect("Unknown SES messageId");

    relay_eml(&mut smtp, &s3, &message_id);
    Ok(Output {})
}

#[tokio::main]
async fn relay_eml(smtp: &mut smtp::SmtpClient, s3: &S3Client, message_id: &str) {
    match s3
        .get_object(GetObjectRequest {
            bucket: env::var("S3_BUCKET").unwrap(),
            key: message_id.into(),
            ..Default::default()
        })
        .then(|obj| obj.unwrap().body.unwrap().into_future())
        .await
    {
        (Some(eml), _) => {
            let email = lettre::Email::new(
                lettre::Envelope::new(
                    Some(
                        lettre::EmailAddress::new(env::var("RELAY_ENVELOPE_FROM").unwrap())
                            .unwrap(),
                    ),
                    vec![
                        lettre::EmailAddress::new(env::var("RELAY_ENVELOPE_TO").unwrap()).unwrap(),
                    ],
                )
                .unwrap(),
                message_id.into(),
                eml.unwrap().as_ref().into(),
            );
            smtp.clone().transport().send(email).unwrap();
        }
        (_, e) => {
            println!("{:#?}", e);
            panic!(e)
        }
    }
}
