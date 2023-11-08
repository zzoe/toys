use reqwest::{Method, Url};

use toy_schema::sign::SignReq;

use crate::error::Result;
use crate::service::{http, SERVER_URL};

pub async fn sign_up(req: SignReq) -> Result<()> {
    let url = Url::parse(SERVER_URL)
        .and_then(|u| u.join("/sign_up"))
        .unwrap();

    http(Method::POST, url, &req).await?;
    Ok(())
}

pub(crate) async fn sign_in(req: SignReq) -> Result<()> {
    let url = Url::parse(SERVER_URL)
        .and_then(|u| u.join("/sign_in"))
        .unwrap();

    http(Method::POST, url, &req).await?;

    Ok(())
}
