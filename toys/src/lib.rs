#![allow(non_snake_case)]

use dioxus::prelude::*;
use dioxus_router::prelude::Router;
use fermi::use_init_atom_root;

use crate::service::api_service;

mod service;
mod ui;

pub fn App(cx: Scope) -> Element {
    let atoms = use_init_atom_root(cx);
    let _api = use_coroutine(cx, |rx| api_service(rx, atoms.clone()));

    render!(Router::<ui::Route> {})
}
