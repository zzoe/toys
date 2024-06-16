use dioxus::prelude::*;
use tracing::info;

#[component]
pub fn Home() -> Element {
    info!("home");
    rsx!(article {
        h1{
            "Home"
        }
    })
}
