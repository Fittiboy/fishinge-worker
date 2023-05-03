#![allow(dead_code)]
pub struct Notification {}

pub struct WebhookCallbackVerification {}

pub struct Revocation {}

#[derive(Debug, Default)]
pub struct TwitchHeaders {
    id: String,
    retry: String,
    message_type: MessageType,
    signature: String,
    timestamp: String,
    subscription_type: String,
    subscription_version: String,
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
