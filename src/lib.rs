#![allow(missing_docs, clippy::missing_panics_doc, clippy::missing_errors_doc)]

use worker::{event, Context, Env, Request, Response, Router, Url};

mod error;
mod handlers;
mod notifications;
mod twitch_auth;
mod utils;
mod verification;

use notifications::{TwitchHeaders, TwitchRequest};
use twitch_auth::{authorization_flow, token_from_code, valid_token};

struct Secrets {
    hmac: String,
    client_id: String,
    client_secret: String,
}

impl Secrets {
    fn retrieve(env: &Env) -> Option<Self> {
        Some(Self {
            hmac: env.secret("HMAC_SECRET").ok()?.to_string(),
            client_id: env.secret("TWITCH_CLIENT_ID").ok()?.to_string(),
            client_secret: env.secret("TWITCH_CLIENT_SECRET").ok()?.to_string(),
        })
    }
}

#[event(fetch)]
pub async fn main(req: Request, env: Env, _ctx: Context) -> worker::Result<Response> {
    utils::log_request(&req);
    utils::set_panic_hook();

    let Some(secrets) = Secrets::retrieve(&env) else {
        return Response::error("Internal server error", 500);
    };
    let router = Router::with_data(secrets);

    router
        .get("/", |_req, _ctx| Response::ok("Hello, pond!"))
        .get("/login", |_req, ctx| {
            authorization_flow("https://fishinge.fitti.io/", &ctx.data.client_id)
        })
        .get_async("/get_token", |req, ctx| async move {
            let url = req.url().unwrap();
            let code = url
                .query_pairs()
                .find_map(|(key, value)| if key == "code" { Some(value) } else { None })
                .ok_or(worker::Error::from("No token received from Twitch"))?;
            let token = token_from_code(
                "https://fishinge.fitti.io/get_token",
                &ctx.data.client_id,
                &ctx.data.client_secret,
                &code,
            )
            .await?;
            if valid_token(&token)
                .await
                .map_err(|err| worker::Error::from(err.to_string()))?
            {
                Response::redirect(
                    Url::parse(&format!(
                        "https://fishinge.fitti.io/dashboard?token={}",
                        token
                    ))
                    .unwrap(),
                )
            } else {
                Response::error("That didn't work somehow! Try contacting Fitti!", 500)
            }
        })
        .get("/dashboard", |req, _ctx| {
            Response::ok(format!("This is how you got here: {}", req.url().unwrap()))
        })
        .post_async("/eventsub", |mut req, ctx| async move {
            let headers: TwitchHeaders = req.headers().try_into()?;
            let body = req.text().await?;
            match webhook(&ctx.data.hmac, &headers, &body) {
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
