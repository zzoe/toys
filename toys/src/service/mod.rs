use std::rc::Rc;
use std::sync::OnceLock;

use bytes::Bytes;
use dioxus::prelude::*;
use fermi::AtomRoot;
use futures_util::stream::StreamExt;
use reqwest::{Client, Method, Url};
use serde::Serialize;

use toy_schema::sign::SignReq;

use crate::error::Error::ResponseError;
use crate::error::Result;

pub static HTTP_CLIENT: OnceLock<Client> = OnceLock::new();
pub static HTTP_URL: OnceLock<Url> = OnceLock::new();

pub mod login;

pub enum Api {
    SignUp(SignReq),
    SignIn(SignReq),
    SignCheck,
}

pub async fn api_service(mut rx: UnboundedReceiver<Api>, atoms: Rc<AtomRoot>) {
    while let Some(msg) = rx.next().await {
        let atoms_clone = atoms.clone();
        match msg {
            Api::SignUp(req) => login::sign_up(atoms_clone, req).await,
            Api::SignIn(req) => login::sign_in(atoms_clone, req).await,
            Api::SignCheck => login::sign_check(atoms_clone).await,
        }
    }
}

pub async fn http<Req: Serialize>(method: Method, path: &str, req: &Req) -> Result<Bytes> {
    let client = HTTP_CLIENT.get().unwrap();
    let url = HTTP_URL.get().and_then(|u| u.join(path).ok()).unwrap();

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
