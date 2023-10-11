use dioxus::prelude::*;
use tracing::info;

pub fn Home(cx: Scope) -> Element {
    info!("home");
    render!(article {
        h1{
            "Home"
        }
    })
}
