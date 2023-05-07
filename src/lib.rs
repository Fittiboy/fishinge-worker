#![allow(missing_docs, clippy::missing_panics_doc, clippy::missing_errors_doc)]

use worker::{event, Context, Env, Request, Response, Router, Url};

mod apis;
mod data;
mod error;
mod html;
mod notifications;
mod twitch_auth;
mod utils;
mod verification;

use apis::User;
use data::{Client, Secrets};
use notifications::{TwitchHeaders, TwitchRequest};
use twitch_auth::{authorization_flow, request_token, valid_token, ValidationResponse};
use utils::Length;

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
        .get("/login", |req, ctx| {
            authorization_flow(req.url()?, &ctx.data.client_id)
        })
        .get_async("/twitch_token", |req, ctx| async move {
            let data = Client::new(&req, &ctx)?;
            let token = request_token(data).await?;
            match valid_token(&token).await? {
                ValidationResponse::Valid(token_metadata) => {
                    let users = ctx.kv("users")?;
                    let mut user = User::from_login(token, token_metadata);
                    let mut redirect: Url = req.url()?;
                    redirect.set_query(None);
                    if user.with_se_token(&users).await.is_some() {
                        users
                            .put(&user.token_metadata.user_id, &user)?
                            .execute()
                            .await?;
                        redirect.set_path(&format!(
                            "/authenticate?token={}",
                            user.token_metadata.user_id
                        ));
                    } else {
                        redirect.set_path("/se_token");
                    }
                    Response::redirect(redirect)
                }
                ValidationResponse::Invalid(_) => {
                    Response::error("That didn't work somehow! Try contacting Fitti!", 500)
                }
            }
        })
        .get("/authenticate", |req, _ctx| {
            Response::ok(format!("This is how you got here: {}", req.url().unwrap()))
        })
        .get("/se_token", |_req, _ctx| {
            let html = html::SE_TOKEN_FORM;
            Response::from_html(html)
        })
        .post_async("/eventsub", |mut req, ctx| async move {
            let headers: &TwitchHeaders = &req.headers().try_into()?;
            let body = &req.text().await?;
            match webhook(&ctx.data.hmac, headers, body) {
                Ok(body) => Response::ok(body).with_length(),
                Err(err) => err.into(),
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
