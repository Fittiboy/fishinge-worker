use worker::Env;

pub struct Secrets {
    pub hmac: String,
    pub client_id: String,
    pub client_secret: String,
}

impl Secrets {
    pub fn retrieve(env: &Env) -> Option<Self> {
        Some(Self {
            hmac: env.secret("HMAC_SECRET").ok()?.to_string(),
            client_id: env.secret("TWITCH_CLIENT_ID").ok()?.to_string(),
            client_secret: env.secret("TWITCH_CLIENT_SECRET").ok()?.to_string(),
        })
    }
}

#[derive(Debug, Clone)]
pub struct Client {
    pub redirect: String,
    pub client_id: String,
    pub client_secret: String,
    pub code: String,
    pub token: String,
}

impl Client {
    pub fn new(req: &worker::Request, ctx: &worker::RouteContext<Secrets>) -> worker::Result<Self> {
        let mut url = req.url().unwrap();
        let code = url
            .query_pairs()
            .find_map(|(key, value)| if key == "code" { Some(value) } else { None })
            .ok_or(worker::Error::from("No token received from Twitch"))?
            .to_string();
        url.set_path("/twitch_token");
        url.set_query(None);
        Ok(Self {
            redirect: url.to_string(),
            client_id: ctx.data.client_id.clone(),
            client_secret: ctx.data.client_secret.clone(),
            code,
            token: String::new(),
        })
    }
}
