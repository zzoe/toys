use std::rc::Rc;
use std::sync::OnceLock;

use bytes::Bytes;
use dioxus::prelude::*;
use fermi::AtomRoot;
use futures_util::stream::StreamExt;
use reqwest::{Client, Method, Url};
use serde::Serialize;
use tracing::error;

use toy_schema::sign::SignReq;

use crate::error::Error::ResponseError;
use crate::error::Result;

pub static HTTP_CLIENT: OnceLock<Client> = OnceLock::new();
pub static SERVER_URL: &str = "http://127.0.0.1:8080/";

pub mod login;

pub enum Api {
    SignUp(SignReq),
    SignIn(SignReq),
}

pub async fn api_service(mut rx: UnboundedReceiver<Api>, _atoms: Rc<AtomRoot>) {
    while let Some(msg) = rx.next().await {
        match msg {
            Api::SignUp(req) => {
                if let Err(e) = login::sign_up(req).await {
                    error!("{e}");
                }
            }
            Api::SignIn(req) => {
                if let Err(e) = login::sign_in(req).await {
                    error!("{e}");
                }
            }
        }
    }
}

pub async fn http<Req: Serialize>(method: Method, url: Url, req: &Req) -> Result<Bytes> {
    let client = HTTP_CLIENT.get().unwrap();

    let res = client.request(method, url).json(req).send().await?;
    let status = res.status();
    let msg = res.bytes().await?;
    if !status.is_success() {
        return Err(ResponseError {
            status,
            msg: String::from_utf8_lossy(&msg).to_string(),
        });
    }

    Ok(msg)
}