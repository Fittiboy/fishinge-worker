use std::{error, fmt};

#[derive(Debug)]
pub enum Webhook {
    CannotVerifyMessage,
}

impl error::Error for Webhook {}

impl fmt::Display for Webhook {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}
