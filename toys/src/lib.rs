#![allow(non_snake_case)]

use dioxus::prelude::*;
use dioxus_router::prelude::Router;

mod ui;

pub fn App(cx: Scope) -> Element {
    render!(Router::<ui::Route> {})
}
