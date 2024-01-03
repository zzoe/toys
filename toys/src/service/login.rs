use reqwest::Method;

use toy_schema::sign::{SignCheck, SignReq};

use crate::error::Result;
use crate::service::http;

pub async fn sign_up(req: SignReq) -> Result<()> {
    http(Method::POST, "/sign_up", &req).await?;
    Ok(())
}

pub async fn sign_in(req: SignReq) -> Result<()> {
    http(Method::POST, "/sign_in", &req).await?;
    Ok(())
}

pub async fn sign_check() -> Result<()> {
    http(Method::POST, "/sign_check", &SignCheck {}).await?;
    Ok(())
}
