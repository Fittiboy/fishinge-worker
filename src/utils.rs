use cfg_if::cfg_if;
use worker::{console_log, Date, Request, Response, ResponseBody};

pub fn log_request(req: &Request) {
    console_log!(
        "{} - [{}], located at: {:?}, within: {}",
        Date::now().to_string(),
        req.path(),
        req.cf().coordinates().unwrap_or_default(),
        req.cf().region().unwrap_or_else(|| "unknown region".into())
    );
}

cfg_if! {
    if #[cfg(feature = "console_error_panic_hook")] {
        extern crate console_error_panic_hook;
        pub use self::console_error_panic_hook::set_once as set_panic_hook;
    } else {
        #[inline]
        pub fn set_panic_hook() {}
    }
}

pub trait Length {
    fn with_length(self) -> Self;
}

impl Length for worker::Result<Response> {
    fn with_length(self) -> Self {
        let mut response = self?;
        if let ResponseBody::Body(body) = response.body() {
            let length = body.len().to_string();
            response.headers_mut().set("Content-Length", &length)?;
        };
        Ok(response)
    }
}
