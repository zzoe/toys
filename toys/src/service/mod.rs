use dioxus::prelude::*;
use fermi::AtomRoot;
use futures_util::stream::StreamExt;
use reqwest::Client;
use std::rc::Rc;
use std::sync::OnceLock;
use tracing::error;

pub static HTTP_CLIENT: OnceLock<Client> = OnceLock::new();
pub static SERVER_URL: &str = "http://127.0.0.1:8080/";

pub mod login;

pub enum Api {
    Login(login::Req),
}

pub async fn api_service(mut rx: UnboundedReceiver<Api>, _atoms: Rc<AtomRoot>) {
    while let Some(msg) = rx.next().await {
        match msg {
            Api::Login(req) => {
                if let Err(e) = login::sign_up(req).await {
                    error!("{e}");
                }
            }
        }
    }
}
