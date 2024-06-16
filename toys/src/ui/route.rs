use dioxus::prelude::*;
use dioxus_router::prelude::*;

use super::config::Settings;
use super::fight_the_landlord::FightTheLandlord;
use super::header::{Breadcrumbs, Header};
use super::home::Home;
use super::menu::{Menu, MenuHidden};
use super::proofreading::Proofreading;
use super::sign::{Sign, AUTHENTICATED};
use super::sudoku::Sudoku;

#[rustfmt::skip]
#[derive(Clone, Debug, PartialEq, Routable)]
pub enum Route {
  #[layout(Body)]
    #[route("/")]
    //  if the current location doesn't match any of the other routes, redirect to "/home"
    #[redirect("/:.._s", |_s: Vec<String>| Route::Home {})]
    Home {},
    #[route("/sudoku")]
    Sudoku {},
    #[route("/fight_the_landlord")]
    FightTheLandlord {},
    #[route("/proofreading")]
    Proofreading {},
    #[route("/settings")]
    Settings {},
}

fn Body() -> Element {
    use_context_provider(|| Signal::new(MenuHidden(false)));
    use_context_provider(|| Signal::new(Breadcrumbs(Vec::new())));

    if AUTHENTICATED() {
        // 登录后
        rsx!(
        div { class: "flex flex-col min-h-screen mx-auto",
            Header{}
            main { class: "grow flex flex-row p-3 space-x-3",
                Menu{}
                Outlet::<Route> {}
            }
            footer { class: "flex flex-row justify-center p-3",
                p { "Copyright © zoe" }
            }
        })
    } else {
        // 登录
        rsx!(Sign {})
    }
}
