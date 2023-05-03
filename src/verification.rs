use crate::notifications::TwitchHeaders;

pub fn good_hmac(secret: String, headers: TwitchHeaders) -> bool {
    const _TWITCH_MESSAGE_ID: &str = "twitch-eventsub-message-id";
    const _TWITCH_MESSAGE_TIMESTAMP: &str = "twitch-eventsub-message-timestamp";
    const _TWITCH_MESSAGE_SIGNATURE: &str = "twitch-eventsub-message-signature";
    const _HMAC_PREFIX: &str = "sha256=";
    drop(secret);
    drop(headers);
    true
}
