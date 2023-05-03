pub fn good_hmac(secret: String) -> bool {
    const twitch_message_id: &str = "twitch-eventsub-message-id";
    const twitch_message_timestamp: &str = "twitch-eventsub-message-timestamp";
    const twitch_message_signature: &str = "twitch-eventsub-message-signature";
    const HMAC_PREFIX: &str = "sha256=";
    true
}
