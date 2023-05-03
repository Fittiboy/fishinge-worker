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
        .post("/eventsub", |req, ctx| {
            let headers: TwitchHeaders = req.headers().try_into()?;
            match webhook(ctx.data, headers) {
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

fn webhook(secret: String, headers: TwitchHeaders) -> Result<String, error::Webhook> {
    if verification::good_hmac(secret, headers) {
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
        webhook("test".to_string(), TwitchHeaders::default()).unwrap();
    }
}
