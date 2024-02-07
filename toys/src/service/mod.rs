use std::rc::Rc;
use std::sync::OnceLock;

use bytes::Bytes;
use dioxus::prelude::*;
use fermi::AtomRoot;
use futures_util::stream::StreamExt;
use reqwest::{Client, Method, StatusCode, Url};
use serde::Serialize;

use toy_schema::sign::SignReq;

use crate::error::Error::ResponseError;
use crate::error::Result;
use crate::ui::sign::AUTHENTICATED;
use crate::ui::unique_id;

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

pub async fn api_service(mut rx: UnboundedReceiver<Api>, atoms: Rc<AtomRoot>) {
    while let Some(msg) = rx.next().await {
        match msg {
            Api::SignUp(req) => sign::sign_up(atoms.as_ref(), req).await,
            Api::SignIn(req) => sign::sign_in(atoms.as_ref(), req).await,
            Api::SignCheck => sign::sign_check(atoms.as_ref()).await,
            Api::ConfigReload => config::reload(atoms.as_ref()).await,
        }
    }
}

pub async fn http<Req: Serialize>(
    atoms: &AtomRoot,
    method: Method,
    path: &str,
    request: Option<&Req>,
) -> Result<Bytes> {
    let client = HTTP_CLIENT.get().unwrap();
    let url = HTTP_URL.get().and_then(|u| u.join(path).ok()).unwrap();

    // 发送请求
    let res = match request {
        Some(req) => client.request(method, url).json(req).send().await?,
        None => client.request(method, url).send().await?,
    };

    let status = res.status();
    let msg = res.bytes().await?;

    if !status.is_success() {
        if StatusCode::UNAUTHORIZED.eq(&status) {
            atoms.set(unique_id(&AUTHENTICATED), false);
        }

        return Err(ResponseError {
            status,
            msg: String::from_utf8_lossy(&msg).to_string(),
        });
    }

    Ok(msg)
}
