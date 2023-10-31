use crate::error::Error::ResponseError;
use reqwest::Url;
use serde::Serialize;
use tracing::error;

use crate::error::Result;
use crate::service::{HTTP_CLIENT, SERVER_URL};

#[derive(Serialize)]
pub struct Req {
    pub name: String,
    pub email: String,
    pub password: String,
}

pub async fn sign_up(req: Req) -> Result<()> {
    let client = HTTP_CLIENT.get().unwrap();
    let url = Url::parse(SERVER_URL)
        .and_then(|u| u.join("/sign_up"))
        .unwrap();

    let res = client.post(url).json(&req).send().await?;
    let status = res.status();
    if !status.is_success() {
        let msg = match res
            .bytes()
            .await
            .map(|b| String::from_utf8_lossy(&b).to_string())
        {
            Ok(msg) => msg,
            Err(e) => {
                error!("http response body to string failed: {e}");
                e.to_string()
            }
        };
        return Err(ResponseError { status, msg });
    }

    Ok(())
}
