use crate::notifications::WebhookCallbackVerification;

pub fn challenge_callback(body: WebhookCallbackVerification) -> String {
    "Pog".to_string()
}
