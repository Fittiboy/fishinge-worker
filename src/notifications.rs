use crate::error;
use serde::Deserialize;
use std::convert::Into;

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum TwitchRequest {
    WebhookCallbackVerification(WebhookCallbackVerification),
    Notification(Notification),
    Revocation(Revocation),
}

impl TryFrom<&str> for TwitchRequest {
    type Error = error::Webhook;

    fn try_from(body: &str) -> Result<Self, Self::Error> {
        serde_json::from_str(body).map_err(Into::into)
    }
}

#[derive(Debug, Deserialize)]
pub struct WebhookCallbackVerification {
    pub challenge: String,
    pub subscription: Subscription,
}

#[derive(Debug, Deserialize)]
pub struct Notification {
    pub subscription: Subscription,
    pub event: Event,
}

#[derive(Debug, Deserialize)]
pub struct Event {
    pub user_id: String,
    pub user_name: String,
}

#[derive(Debug, Deserialize)]
pub struct Subscription {
    pub id: String,
    pub r#type: NotificationType,
    pub status: String,
    pub condition: Condition,
    pub created_at: String,
}

#[derive(Debug, Deserialize)]
pub enum NotificationType {
    #[serde(rename = "channel.channel_points_custom_reward_redemption.add")]
    Redemption,
    #[serde(rename = "channel.follow")]
    Follow,
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum Condition {
    Redemption(RedemptionCondition),
    Follow(FollowCondition),
}

#[derive(Debug, Deserialize)]
pub struct RedemptionCondition {
    pub broadcaster_user_id: String,
    pub reward_id: String,
}

#[derive(Debug, Deserialize)]
pub struct FollowCondition {
    pub broadcaster_user_id: String,
}

#[derive(Debug, Deserialize)]
pub struct Revocation {
    pub subscription: Subscription,
}

#[derive(Debug, Default)]
pub struct TwitchHeaders {
    pub id: String,
    pub retry: String,
    pub message_type: MessageType,
    pub signature: String,
    pub timestamp: String,
    pub subscription_type: String,
    pub subscription_version: String,
}

impl TryFrom<&worker::Headers> for TwitchHeaders {
    type Error = worker::Error;

    fn try_from(headers: &worker::Headers) -> Result<Self, Self::Error> {
        Ok(TwitchHeaders {
            id: grab_header(headers, "Twitch-Eventsub-Message-Id")?,
            retry: grab_header(headers, "Twitch-Eventsub-Message-Retry")?,
            message_type: MessageType::try_from(grab_header(
                headers,
                "Twitch-Eventsub-Message-Type",
            )?)?,
            signature: grab_header(headers, "Twitch-Eventsub-Message-Signature")?,
            timestamp: grab_header(headers, "Twitch-Eventsub-Message-Timestamp")?,
            subscription_type: grab_header(headers, "Twitch-Eventsub-Subscription-Type")?,
            subscription_version: grab_header(headers, "Twitch-Eventsub-Subscription-Version")?,
        })
    }
}

fn grab_header(headers: &worker::Headers, name: &str) -> Result<String, worker::Error> {
    match headers.get(name) {
        Ok(Some(header)) => Ok(header),
        Ok(None) => Err(worker::Error::from("Couldn't read headers")),
        Err(err) => Err(err),
    }
}

#[derive(Debug, Default)]
pub enum MessageType {
    #[default]
    Notification,
    WebhookCallbackVerification,
    Revocation,
}

impl TryFrom<String> for MessageType {
    type Error = worker::Error;

    fn try_from(name: String) -> Result<Self, Self::Error> {
        match name.as_str() {
            "notification" => Ok(Self::Notification),
            "webhook_callback_verification" => Ok(Self::WebhookCallbackVerification),
            "revocation" => Ok(Self::Revocation),
            _ => Err(worker::Error::from("Couldn't read message type")),
        }
    }
}
