use dioxus::prelude::*;
use dioxus_router::prelude::*;
use fermi::{use_read, Atom};

use crate::ui::home::Home;
use crate::ui::menu::Menu;
use crate::ui::sign::Sign;

mod home;
mod menu;
mod sign;

pub static AUTHENTICATED: Atom<bool> = Atom(|_| false);

#[rustfmt::skip]
#[derive(Clone, Debug, PartialEq, Routable)]
pub enum Route {
  #[layout(Body)]
    #[route("/")]
    Home {}
}

fn Body(cx: Scope) -> Element {
    let menu_hidden = use_state(cx, || true);
    let authenticated = use_read(cx, &AUTHENTICATED);

    if *authenticated {
        // 登录后
        render!(
            div { class: "flex flex-col min-h-screen",
                header { class: "p-3",
                    div { class: "flex flex-row p-3 space-x-3 items-center",
                        img {
                            width: 50,
                            src: "rustacean-orig-noshadow.svg",
                            onclick: |_| menu_hidden.set(!*menu_hidden.get()),
                        }
                        Link { to: Route::Home {}, "首页" }
                    }
                    nav { h1 { "Hello" } }
                }
                main { class: "grow flex flex-row p-3 space-x-3",
                    Menu{hidden: *menu_hidden.get()}
                    Outlet::<Route> {}
                }
                footer { class: "flex flex-row justify-center p-3", p { "Copyright © zoe" } }
            }
        )
    } else {
        // 未登录
        render!(Sign {})
    }
}
