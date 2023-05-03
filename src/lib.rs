#![allow(missing_docs, clippy::missing_errors_doc)]

use worker::{event, Context, Env, Request, Response, Router};

mod error;
mod notifications;
mod utils;
mod verification;

use notifications::TwitchHeaders;

#[event(fetch)]
pub async fn main(req: Request, env: Env, _ctx: Context) -> worker::Result<Response> {
    utils::log_request(&req);
    utils::set_panic_hook();

    let Ok(secret) = env.secret("HMAC_SECRET") else {
        return Response::error("Internal server error", 500);
    };
    let router = Router::with_data(secret.to_string());

    router
        .get("/", |_req, _ctx| Response::ok("Hello, pond!"))
        .post_async("/eventsub", |mut req, ctx| async move {
            let headers: TwitchHeaders = req.headers().try_into()?;
            let body = req.text().await?;
            match webhook(&ctx.data, &headers, &body) {
                Ok(body) => Response::ok(body),
                Err(err) => Response::error(
                    err.to_string(),
                    match err {
                        error::Webhook::CannotVerifyMessage => 403,
                    },
                ),
            }
        })
        .run(req, env)
        .await
}

fn webhook(secret: &str, headers: &TwitchHeaders, body: &str) -> Result<String, error::Webhook> {
    if verification::good_signature(secret, headers, body) {
        Ok("Hello, pond!".to_string())
    } else {
        Err(error::Webhook::CannotVerifyMessage)
    }
}

#[cfg(test)]
pub mod test {
    use super::*;

    #[test]
    fn webhook_ok() {
        let headers = TwitchHeaders {
            id: "ID".to_string(),
            timestamp: "TIMESTAMP".to_string(),
            signature: "sha256=6883f95e18aaf4ff2e84bbbf47640f2ab719c18f92bba0b595146a11e6a49aef"
                .to_string(),
            ..Default::default()
        };
        webhook("test", &headers, "BODY").unwrap();
    }

    #[test]
    #[should_panic]
    fn error_on_bad_signature() {
        let headers = TwitchHeaders {
            id: "ID".to_string(),
            timestamp: "TIMESTAMP".to_string(),
            signature: "sha256=6883f95e18aaf4ff2e84bbbf47640f2ab719c18f92bba0b595146a11e6a49ae" //truncated
                .to_string(),
            ..Default::default()
        };
        webhook("test", &headers, "BODY").unwrap();
    }
}
