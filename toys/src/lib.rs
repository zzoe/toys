#![allow(non_snake_case)]

use dioxus::prelude::*;
use dioxus_router::prelude::Router;
use reqwest::{ClientBuilder, Url};

use crate::service::{api_service, HTTP_CLIENT, HTTP_URL};

mod error;
mod service;
mod ui;

pub fn init() {
    const LOCAL_HOST: &str = "https://127.0.0.1:8080";

    HTTP_CLIENT.get_or_init(|| ClientBuilder::new().build().unwrap());
    let host = web_sys::window()
        .and_then(|w| w.location().href().ok())
        .unwrap_or(LOCAL_HOST.to_string());
    HTTP_URL.get_or_init(|| Url::parse(&host).unwrap());
}

pub fn App() -> Element {
    use_coroutine(api_service);

    rsx!(Router::<ui::Route> {})
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
