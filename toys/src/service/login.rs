use reqwest::{Method, Url};

use toy_schema::sign::{SignCheck, SignReq};

use crate::error::Result;
use crate::service::{http, SERVER_URL};

pub async fn sign_up(req: SignReq) -> Result<()> {
    let url = Url::parse(SERVER_URL)
        .and_then(|u| u.join("/sign_up"))
        .unwrap();

    http(Method::POST, url, &req).await?;
    Ok(())
}

pub async fn sign_in(req: SignReq) -> Result<()> {
    let url = Url::parse(SERVER_URL)
        .and_then(|u| u.join("/sign_in"))
        .unwrap();

    http(Method::POST, url, &req).await?;

    Ok(())
}

pub async fn sign_check() -> Result<()> {
    let url = Url::parse(SERVER_URL)
        .and_then(|u| u.join("/sign_check"))
        .unwrap();

    http(Method::POST, url, &SignCheck {}).await?;

    Ok(())
}
