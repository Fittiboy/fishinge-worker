use hmac::{Hmac, Mac};
use sha2::Sha256;

type HmacSha256 = Hmac<Sha256>;

use crate::notifications::TwitchHeaders;

pub fn good_signature(secret: &str, headers: &TwitchHeaders, body: &str) -> bool {
    const _HMAC_PREFIX: &str = "sha256=";
    let message = construct_hmac_message(headers, body);
    let signature = headers.signature.clone();
    let signature = hex::decode(signature.as_str().split_once('=').unwrap().1).unwrap();
    verify_signature(secret, &message, signature.as_slice())
}

fn construct_hmac_message(headers: &TwitchHeaders, body: &str) -> Vec<u8> {
    let mut message = Vec::with_capacity(headers.id.len() + headers.timestamp.len() + body.len());
    message.extend_from_slice(headers.id.as_bytes());
    message.extend_from_slice(headers.timestamp.as_bytes());
    message.extend_from_slice(body.as_bytes());
    message
}

fn verify_signature(secret: &str, message: &Vec<u8>, signature: &[u8]) -> bool {
    let mut mac =
        HmacSha256::new_from_slice(secret.as_bytes()).expect("HMAC can take key of any size");
    mac.update(message.as_slice());
    mac.verify_slice(signature).is_ok()
}
