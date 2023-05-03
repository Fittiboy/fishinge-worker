#![allow(missing_docs, clippy::missing_errors_doc)]
use std::{error, fmt};

use worker::{event, Context, Env, Request, Response, Router};

mod notifications;
mod utils;
mod verification;

#[event(fetch)]
pub async fn main(req: Request, env: Env, _ctx: Context) -> worker::Result<Response> {
    let secret = env
        .secret("HMAC_SECRET")
        .expect("HMAC_SECRET needs to be defined for message verification")
        .to_string();
    utils::log_request(&req);
    utils::set_panic_hook();
    let router = Router::with_data(secret);

    router
        .get("/", |_req, ctx| match webhook(ctx.data) {
            Ok(body) => Response::ok(body),
            Err(err) => Response::error(
                err.to_string(),
                match err {
                    WebhookError::CannotVerifyMessage => 403,
                },
            ),
        })
        .run(req, env)
        .await
}

pub fn webhook(secret: String) -> Result<String, WebhookError> {
    if verification::good_hmac(secret) {
        Ok("Hello, pond!".to_string())
    } else {
        Err(WebhookError::CannotVerifyMessage)
    }
}

#[derive(Debug)]
pub enum WebhookError {
    CannotVerifyMessage,
}

impl error::Error for WebhookError {}

impl fmt::Display for WebhookError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[cfg(test)]
pub mod test {
    use super::*;

    #[test]
    fn webhook_ok() {
        let _response = webhook("test".to_string()).unwrap();
    }
}
