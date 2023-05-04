use std::{error, fmt};

#[derive(Debug)]
pub enum Webhook {
    CannotVerifyMessage,
    CannotParseBody(serde_json::Error),
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
