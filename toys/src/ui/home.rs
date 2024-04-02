use dioxus::prelude::*;
use tracing::info;

pub fn Home() -> Element {
    info!("home");
    rsx!(article {
        h1{
            "Home"
        }
    })
}
