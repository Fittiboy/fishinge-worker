#![allow(missing_docs, clippy::missing_panics_doc, clippy::missing_errors_doc)]

use worker::{event, Context, Env, Request, Response, Router};

mod error;
mod handlers;
mod notifications;
mod utils;
mod verification;

use notifications::{TwitchHeaders, TwitchRequest};

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
                Ok(body) => {
                    let content_length = body.as_bytes().len();
                    let mut response = Response::ok(body).unwrap();
                    let headers = response.headers_mut();
                    headers
                        .set("Content-Length", &content_length.to_string())
                        .unwrap();
                    Ok(response)
                }
                Err(err) => Response::error(
                    err.to_string(),
                    match err {
                        error::Webhook::CannotVerifyMessage => 403,
                        error::Webhook::CannotParseBody(_) => 500,
                    },
                ),
            }
        })
        .run(req, env)
        .await
}

fn webhook(secret: &str, headers: &TwitchHeaders, body: &str) -> Result<String, error::Webhook> {
    if !verification::good_signature(secret, headers, body) {
        return Err(error::Webhook::CannotVerifyMessage);
    }
    let body: TwitchRequest = body.try_into()?;
    let response = match body {
        TwitchRequest::WebhookCallbackVerification(body) => body.challenge,
        TwitchRequest::Notification(_) => "Thanks for the notification!".to_string(),
        TwitchRequest::Revocation(_) => "Sad to see you go!".to_string(),
    };
    Ok(response)
}

#[cfg(test)]
mod test;
