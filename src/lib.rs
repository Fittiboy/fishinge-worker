#![allow(missing_docs, clippy::missing_errors_doc)]

use worker::{event, Context, Env, Request, Response, Router};

mod utils;

#[event(fetch)]
pub async fn main(req: Request, env: Env, _ctx: Context) -> worker::Result<Response> {
    utils::log_request(&req);
    utils::set_panic_hook();
    let router = Router::new();

    router
        .get("/", |_, _| Response::ok("Hello, pond!"))
        .run(req, env)
        .await
}
