#![allow(missing_docs, clippy::missing_errors_doc)]
use std::{error, fmt};

use worker::{event, Context, Env, Request, Response, Router};

mod utils;

#[event(fetch)]
pub async fn main(req: Request, env: Env, _ctx: Context) -> worker::Result<Response> {
    utils::log_request(&req);
    utils::set_panic_hook();
    let router = Router::new();

    router
        .get("/", |_req, _ctx| match webhook() {
            Ok(body) => Response::ok(body),
            Err(err) => Response::error(err.to_string(), 500),
        })
        .run(req, env)
        .await
}

pub fn webhook() -> Result<String, WebhookError> {
    Ok("Hello, pond!".to_string())
}

#[derive(Debug)]
pub enum WebhookError {}

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
        let _response = webhook().unwrap();
    }
}
