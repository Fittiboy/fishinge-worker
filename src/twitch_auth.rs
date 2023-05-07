use serde::{Deserialize, Serialize};
use std::str::FromStr;
use worker::Url;

use crate::apis::UserAccessToken;
use crate::data::Client;
use crate::error::Twitch;

pub fn authorization_flow(redirect: &str, client_id: &str) -> worker::Result<worker::Response> {
    let redirect: Url = Url::from_str(redirect).unwrap();
    let redirect = redirect.join("/get_token")?;
    let scopes = Scopes::from(&["channel:manage:redemptions"]);
    let mut auth_url = Url::parse("https://id.twitch.tv/oauth2/authorize")?;
    auth_url
        .query_pairs_mut()
        .extend_pairs(&[
            ("client_id", client_id),
            ("redirect_uri", redirect.as_str()),
            ("response_type", "code"),
            ("scope", scopes.0.as_str()),
        ])
        .finish();
    worker::Response::redirect(auth_url)
}

pub async fn request_token(data: Client) -> Result<UserAccessToken, Twitch> {
    let params = [
        ("client_id", data.client_id),
        ("client_secret", data.client_secret),
        ("code", data.code),
        ("grant_type", "authorization_code".to_string()),
        ("redirect_uri", data.redirect),
    ];
    let client = reqwest::Client::new();
    let response: UserAccessTokenResponse = client
        .post("https://id.twitch.tv/oauth2/token")
        .form(&params)
        .send()
        .await?
        .json()
        .await?;

    Ok(response.access_token)
}

#[derive(Debug, Deserialize)]
pub struct UserAccessTokenResponse {
    pub access_token: UserAccessToken,
    pub expires_in: usize,
    pub refresh_token: String,
    pub scope: Vec<String>,
    pub token_type: String,
}

pub async fn valid_token(token: &str) -> Result<ValidationResponse, Twitch> {
    let client = reqwest::Client::new();
    Ok(client
        .get("https://id.twitch.tv/oauth2/validate")
        .header("Authorization", format!("Bearer {}", token))
        .send()
        .await?
        .json()
        .await?)
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum ValidationResponse {
    Valid(TokenMetadata),
    Invalid(InvalidToken),
}

#[derive(Debug, Deserialize, Serialize)]
pub struct TokenMetadata {
    pub client_id: String,
    pub login: String,
    pub scopes: Vec<String>,
    pub user_id: String,
    pub expires_in: usize,
}

#[derive(Debug, Deserialize)]
pub struct InvalidToken {
    pub status: usize,
    pub message: String,
}

//TODO: remove annotation after using function
#[allow(dead_code)]
pub async fn app_access_token(client_id: &str, client_secret: &str) -> Result<String, Twitch> {
    let params = [
        ("client_id", client_id),
        ("client_secret", client_secret),
        ("grant_type", "client_credentials"),
    ];
    let client = reqwest::Client::new();
    let response: AppAccessTokenResponse = client
        .post("https://id.twitch.tv/oauth2/token")
        .form(&params)
        .send()
        .await?
        .json()
        .await?;

    Ok(response.access_token)
}

#[derive(Debug, Serialize)]
struct Scopes(String);

impl Scopes {
    fn from(scopes: &[&str]) -> Self {
        Self(scopes.join(" "))
    }
}

#[derive(Debug, Deserialize)]
pub struct AppAccessTokenResponse {
    pub access_token: String,
    pub expires_in: usize,
    pub token_type: String,
}
