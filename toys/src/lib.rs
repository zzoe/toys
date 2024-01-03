#![allow(non_snake_case)]

use dioxus::prelude::*;
use dioxus_router::prelude::Router;
use fermi::use_init_atom_root;
use reqwest::{ClientBuilder, Url};
use tracing::info;

use crate::service::{api_service, HTTP_CLIENT, HTTP_URL};

mod error;
mod service;
mod ui;

pub fn init() {
    const LOCAL_HOST: &str = "http://127.0.0.1:8080";
    #[cfg(not(target_arch = "wasm32"))]
    {
        HTTP_CLIENT.get_or_init(|| ClientBuilder::new().cookie_store(true).build().unwrap());
        HTTP_URL.get_or_init(|| Url::parse(LOCAL_HOST).unwrap());
    }

    #[cfg(target_arch = "wasm32")]
    {
        HTTP_CLIENT.get_or_init(|| ClientBuilder::new().build().unwrap());
        let host = web_sys::window()
            .and_then(|w| w.location().href().ok())
            .unwrap_or(LOCAL_HOST.to_string());
        HTTP_URL.get_or_init(|| Url::parse(&host).unwrap());
    }
}

pub fn App(cx: Scope) -> Element {
    let atoms = use_init_atom_root(cx);
    let _api = use_coroutine(cx, |rx| api_service(rx, atoms.clone()));

    render!(Router::<ui::Route> {})
}

#[cfg(test)]
mod test {
    use reqwest::Url;

    #[test]
    fn url_parse() {
        let url = Url::parse("http://172.65.11.22:8880");
        println!("{url:#?}");
        assert!(url.is_ok());

        let dst = url.unwrap().join("/abc/def");
        println!("{dst:#?}");
        assert!(dst.is_ok());
    }
}
