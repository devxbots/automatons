use anyhow::Context;
use axum::body::Bytes;
use axum::extract::State;
use axum::http::{HeaderMap, StatusCode};
use hmac::{Hmac, Mac};
use secrecy::ExposeSecret;
use sha2::Sha256;

use automatons_github::event::GitHubEvent;
use aws_sdk_sqs::{Client, Config};

use crate::error::{Error, Result};
use crate::{AppState, GitHubWebhookSecret};

type HmacSha256 = Hmac<Sha256>;

pub async fn github_webhook_handler(
    State(app_state): State<AppState>,
    headers: HeaderMap,
    body: Bytes,
) -> Result<StatusCode> {
    let signature = get_signature(&headers)?;
    verify_signature(&body, &signature, &app_state.github_webhook_secret)?;

    let event_type = get_event(&headers)?;
    let event = deserialize_event(&event_type, &body)?;

    let serialized_event =
        serde_json::to_string(&event).context("failed to serialize GitHub event")?;

    Client::from_conf(Config::from(&app_state.aws_configuration))
        .send_message()
        .queue_url(app_state.aws_event_queue_url)
        .message_body(serialized_event)
        .send()
        .await
        .unwrap();
    // .context("failed to queue GitHub event")?;

    Ok(StatusCode::CREATED)
}

fn get_signature(headers: &HeaderMap) -> Result<String> {
    get_header(headers, "X-Hub-Signature-256")
}

fn get_header(headers: &HeaderMap, header: &str) -> Result<String> {
    headers
        .get(header)
        .and_then(|header| header.to_str().ok())
        .map(String::from)
        .ok_or_else(|| Error::BadRequest(format!("missing {} header", header)))
}

fn verify_signature(body: &Bytes, signature: &str, secret: &GitHubWebhookSecret) -> Result<()> {
    let mut hmac = HmacSha256::new_from_slice(secret.0.expose_secret().as_bytes())
        .context("failed to initialize cryptographic key")?;

    let signature = signature.split('=').last().ok_or_else(|| {
        Error::BadRequest("X-Hub-Signature-256 header has the wrong format".into())
    })?;

    let decoded_signature = hex::decode(signature)
        .map_err(|_| Error::BadRequest("failed to decode the X-Hub-Signature-256 header".into()))?;

    hmac.update(body);
    hmac.verify_slice(decoded_signature.as_slice())
        .map_err(|_| Error::Unauthorized("X-Hub-Signature-256 header is invalid".into()))?;

    Ok(())
}

fn get_event(headers: &HeaderMap) -> Result<String> {
    get_header(headers, "X-GitHub-Event")
}

fn deserialize_event(event_type: &str, body: &Bytes) -> Result<GitHubEvent> {
    let event = match event_type {
        "check_run" => GitHubEvent::CheckRun(
            serde_json::from_slice(body).context("failed to deserialize check_run event")?,
        ),
        _ => {
            // TODO: Log unsupported event type
            GitHubEvent::Unsupported
        }
    };

    Ok(event)
}

#[cfg(test)]
mod tests {
    use axum::body::Bytes;
    use secrecy::SecretString;

    use super::verify_signature;
    use super::GitHubWebhookSecret;

    #[test]
    fn verify_signature_with_valid_signature() {
        let body = "verify_signature";
        let signature = "sha256=22568b39613009e6d1b1fd063085c05063998bda5243a597c0cc524e044990ae";
        let secret = GitHubWebhookSecret(SecretString::new("verify_signature".into()));

        assert!(verify_signature(&Bytes::from(body), signature, &secret).is_ok());
    }

    #[test]
    fn verify_signature_with_empty_body() {
        let body = "";
        let signature = "sha256=22568b39613009e6d1b1fd063085c05063998bda5243a597c0cc524e044990ae";
        let secret = GitHubWebhookSecret(SecretString::new("verify_signature".into()));

        assert!(verify_signature(&Bytes::from(body), signature, &secret).is_err());
    }

    #[test]
    fn verify_signature_with_empty_signature() {
        let body = "verify_signature";
        let signature = "";
        let secret = GitHubWebhookSecret(SecretString::new("verify_signature".into()));

        assert!(verify_signature(&Bytes::from(body), signature, &secret).is_err());
    }

    #[test]
    fn verify_signature_with_empty_body_secret_and_signature() {
        let body = "";
        let signature = "";
        let secret = GitHubWebhookSecret(SecretString::new("".into()));

        assert!(verify_signature(&Bytes::from(body), signature, &secret).is_err());
    }
}
