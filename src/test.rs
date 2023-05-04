use super::*;

fn headers() -> TwitchHeaders {
    TwitchHeaders {
        id: "ID".to_string(),
        timestamp: "TIMESTAMP".to_string(),
        signature: String::new(), // https://cryptotools.net/hmac
        ..Default::default()
    }
}

#[test]
#[should_panic]
fn error_on_bad_signature() {
    let mut headers = headers();
    headers.signature = "sha256=badsignature".to_string();
    webhook("test", &headers, "BODY").unwrap();
}

#[test]
fn responds_to_challenge() {
    let challenge = r#"{
        "challenge": "pogchamp-kappa-360noscope-vohiyo",
        "subscription": {
            "id": "f1c2a387-161a-49f9-a165-0f21d7a4e1c4",
            "status": "webhook_callback_verification_pending",
            "type": "channel.follow",
            "version": "1",
            "cost": 1,
            "condition": {
              "broadcaster_user_id": "12826"
            },
            "transport": {
              "method": "webhook",
              "callback": "https://example.com/webhooks/callback"
            },
            "created_at": "2019-11-16T10:11:12.634234626Z"
        }
    }"#;
    let mut headers = headers();
    headers.message_type = notifications::MessageType::WebhookCallbackVerification;
    headers.signature =
        "sha256=5a7e917cb13a4f91bd6ad5c0486c7019f240d91356e81a17e0f76843faa2f0d9".to_string();
    let response = webhook("callback", &headers, challenge).unwrap();
    assert_eq!(response, "pogchamp-kappa-360noscope-vohiyo".to_string());
}
