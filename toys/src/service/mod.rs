use dioxus::prelude::*;
use fermi::AtomRoot;
use futures_util::stream::StreamExt;
use std::rc::Rc;
use tracing::info;

pub mod login;

pub enum Api {
    Login(String),
}

pub async fn api_service(mut rx: UnboundedReceiver<Api>, _atoms: Rc<AtomRoot>) {
    while let Some(msg) = rx.next().await {
        match msg {
            Api::Login(name) => {
                info!("api call");
            }
        }
    }
}
