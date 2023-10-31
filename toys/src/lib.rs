#![allow(non_snake_case)]

use dioxus::prelude::*;
use dioxus_router::prelude::Router;
use fermi::use_init_atom_root;
use reqwest::ClientBuilder;

use crate::service::{api_service, HTTP_CLIENT};

mod error;
mod service;
mod ui;

pub fn init() {
    #[cfg(not(target_arch = "wasm32"))]
    HTTP_CLIENT.get_or_init(|| ClientBuilder::new().cookie_store(true).build().unwrap());

    #[cfg(target_arch = "wasm32")]
    HTTP_CLIENT.get_or_init(|| ClientBuilder::new().build().unwrap());
}

pub fn App(cx: Scope) -> Element {
    let atoms = use_init_atom_root(cx);
    let _api = use_coroutine(cx, |rx| api_service(rx, atoms.clone()));

    render!(Router::<ui::Route> {})
}
