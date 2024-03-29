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
            "type": "channel.channel_points_custom_reward_redemption.add",
            "version": "1",
            "cost": 1,
            "condition": {
                "broadcaster_user_id": "12826",
                "reward_id": "239847"
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
        "sha256=10ec72721a3462ff6e82bdfdb837973cfd5a8a81e664c3c8b40974ee7ff69b55".to_string();
    let response = webhook("callback", &headers, challenge).unwrap();
    assert_eq!(response, "pogchamp-kappa-360noscope-vohiyo".to_string());
}

#[test]
fn responds_to_notifiaction() {
    let challenge = r#"{
        "subscription": {
            "id": "f1c2a387-161a-49f9-a165-0f21d7a4e1c4",
            "status": "enabled",
            "type": "channel.channel_points_custom_reward_redemption.add",
            "version": "1",
            "cost": 1,
            "condition": {
                "broadcaster_user_id": "12826",
                "reward_id": "239847"
            },
            "transport": {
                "method": "webhook",
                "callback": "https://example.com/webhooks/callback"
            },
            "created_at": "2019-11-16T10:11:12.634234626Z"
        },
        "event": {
            "user_id": "1337",
            "user_login": "awesome_user",
            "user_name": "Awesome_User",
            "broadcaster_user_id":     "12826",
            "broadcaster_user_login":  "twitch",
            "broadcaster_user_name":   "Twitch",
            "followed_at": "2020-07-15T18:16:11.17106713Z"
        }
    }"#;
    let mut headers = headers();
    headers.message_type = notifications::MessageType::WebhookCallbackVerification;
    headers.signature =
        "sha256=ddcf1796ba64df4e6f911e412cce7b37733a43b71b61c58b80ff25a972b31cbf".to_string();
    webhook("notification", &headers, challenge).unwrap();
}

#[test]
fn responds_to_revocation() {
    let challenge = r#"{
        "subscription": {
            "id": "f1c2a387-161a-49f9-a165-0f21d7a4e1c4",
            "status": "authorization_revoked",
            "type": "channel.channel_points_custom_reward_redemption.add",
            "cost": 1,
            "version": "1",
            "condition": {
                "broadcaster_user_id": "12826",
                "reward_id": "239847"
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
        "sha256=e952190b726d6100863096e5982a899f2f70506b01edba8203eba3b80c2a2525".to_string();
    webhook("revocation", &headers, challenge).unwrap();
}
