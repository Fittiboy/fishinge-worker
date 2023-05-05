use std::{error, fmt};

#[derive(Debug)]
pub enum Webhook {
    CannotVerifyMessage,
    CannotParseBody(serde_json::Error),
}

impl Webhook {
    pub fn code(&self) -> u16 {
        match self {
            Self::CannotVerifyMessage => 403,
            Self::CannotParseBody(_) => 500,
        }
    }
}

impl From<Webhook> for worker::Result<worker::Response> {
    fn from(err: Webhook) -> Self {
        worker::Response::error(err.to_string(), err.code())
    }
}

impl error::Error for Webhook {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match self {
            Webhook::CannotVerifyMessage => None,
            Webhook::CannotParseBody(err) => Some(err),
        }
    }
}

impl fmt::Display for Webhook {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl From<serde_json::Error> for Webhook {
    fn from(err: serde_json::Error) -> Self {
        Self::CannotParseBody(err)
    }
}

#[derive(Debug)]
pub enum Twitch {
    CannotGetAccessToken(reqwest::Error),
}

impl error::Error for Twitch {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match self {
            Twitch::CannotGetAccessToken(err) => Some(err),
        }
    }
}

impl fmt::Display for Twitch {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl From<reqwest::Error> for Twitch {
    fn from(err: reqwest::Error) -> Self {
        Self::CannotGetAccessToken(err)
    }
}

impl From<Twitch> for worker::Error {
    fn from(err: Twitch) -> Self {
        Self::from(err.to_string())
    }
}
