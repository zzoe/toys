use dioxus::prelude::*;
use dioxus_router::prelude::*;

use crate::ui::home::Home;

mod home;

#[rustfmt::skip]
#[derive(Clone, Debug, PartialEq, Routable)]
pub enum Route {
  #[layout(Body)]
    #[route("/")]
    Home {}
}

fn Body(cx: Scope) -> Element {
    let nav_hidden = use_state(cx, || "hidden");
    render!(
        div { class: "flex flex-col min-h-screen",
            header { class: "p-3",
                div { class: "flex flex-row p-3 space-x-3 items-center",
                    img {
                        width: 50,
                        src: "rustacean-orig-noshadow.svg",
                        onclick: |_| if nav_hidden.is_empty() { nav_hidden.set("hidden") } else { nav_hidden.set("") }
                    }
                    Link { to: Route::Home {}, "首页" }
                }
                nav { h1 { "Hello" } }
            }
            main { class: "grow flex flex-row p-3 space-x-3",
                nav { class: "flex flex-col p-3 space-y-3 min-w-fit {nav_hidden}",
                    label { "1" }
                    label { "2" }
                }
                Outlet::<Route> {}
            }
            footer { class: "flex flex-row justify-center p-3", p { "© 2023 zoe" } }
        }
    )
}
