use serde::{Deserialize, Serialize};

use crate::twitch_auth::TokenMetadata;

pub type UserAccessToken = String;
pub type SEToken = String;

#[derive(Deserialize, Serialize)]
pub struct User {
    pub access_token: UserAccessToken,
    pub token_metadata: TokenMetadata,
    pub se_token: Option<SEToken>,
}

impl User {
    pub fn from_login(access_token: UserAccessToken, token_metadata: TokenMetadata) -> Self {
        Self {
            access_token,
            token_metadata,
            se_token: None,
        }
    }

    pub async fn with_se_token(&mut self, users: &worker::kv::KvStore) -> Option<()> {
        match users.get(&self.token_metadata.user_id).json().await {
            Ok(Some(User {
                se_token: Some(se_token),
                ..
            })) => self.se_token = Some(se_token),
            _ => return None,
        };
        Some(())
    }
}
