use std::sync::OnceLock;

use bytes::Bytes;
use dioxus::prelude::*;
use futures_util::stream::StreamExt;
use reqwest::{Client, Method, StatusCode, Url};
use speedy::{LittleEndian, Writable};

use toy_schema::sign::SignReq;

use crate::error::Error::ResponseError;
use crate::error::{Error, Result};
use crate::ui::sign::AUTHENTICATED;

pub static HTTP_CLIENT: OnceLock<Client> = OnceLock::new();
pub static HTTP_URL: OnceLock<Url> = OnceLock::new();

mod config;
mod sign;

pub enum Api {
    SignUp(SignReq),
    SignIn(SignReq),
    SignCheck,
    ConfigReload,
}

pub async fn api_service(mut rx: UnboundedReceiver<Api>) {
    while let Some(msg) = rx.next().await {
        match msg {
            Api::SignUp(req) => sign::sign_up(req).await,
            Api::SignIn(req) => sign::sign_in(req).await,
            Api::SignCheck => sign::sign_check().await,
            Api::ConfigReload => config::reload().await,
        }
    }
}

pub async fn http<Req: Writable<LittleEndian>>(
    method: Method,
    path: &str,
    request: Option<&Req>,
) -> Result<Bytes> {
    let client = HTTP_CLIENT.get().unwrap();
    let url = HTTP_URL.get().and_then(|u| u.join(path).ok()).unwrap();

    // 发送请求
    let res = match request {
        Some(req) => {
            let body = req.write_to_vec().map_err(Error::ParseError)?;
            client
                .request(method, url)
                .header("content-type", "application/octet-stream")
                .body(body)
                .send()
                .await?
        }
        None => client.request(method, url).send().await?,
    };

    let status = res.status();
    let msg = res.bytes().await?;

    if !status.is_success() {
        if StatusCode::UNAUTHORIZED.eq(&status) {
            *AUTHENTICATED.write() = false;
        }

        return Err(ResponseError {
            status,
            msg: String::from_utf8_lossy(&msg).to_string(),
        });
    }

    Ok(msg)
}
