use dioxus::prelude::*;
use dioxus_router::prelude::*;
use fermi::Atom;

use crate::ui::home::Home;
use crate::ui::menu::Menu;

mod home;
mod menu;

pub static USER_NAME: Atom<String> = Atom(|_| "default".to_string());

#[rustfmt::skip]
#[derive(Clone, Debug, PartialEq, Routable)]
pub enum Route {
  #[layout(Body)]
    #[route("/")]
    Home {}
}

fn Body(cx: Scope) -> Element {
    let nav_hidden = use_state(cx, || true);

    render!(
        div { class: "flex flex-col min-h-screen",
            header { class: "p-3",
                div { class: "flex flex-row p-3 space-x-3 items-center",
                    img {
                        width: 50,
                        src: "rustacean-orig-noshadow.svg",
                        onclick: |_| nav_hidden.set(!nav_hidden.get()),
                    }
                    Link { to: Route::Home {}, "首页" }
                }
                nav { h1 { "Hello" } }
            }
            main { class: "grow flex flex-row p-3 space-x-3",
                Menu{hidden: *nav_hidden.get()}
                Outlet::<Route> {}
            }
            footer { class: "flex flex-row justify-center p-3", p { "Copyright © zoe" } }
        }
    )
}
