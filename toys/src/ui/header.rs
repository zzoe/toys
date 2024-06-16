use dioxus::prelude::*;
use dioxus_router::prelude::*;

use crate::service::Api;
use crate::ui::menu::MenuHidden;
use crate::ui::route::Route;

pub struct Breadcrumbs(pub Vec<String>);

impl From<Vec<String>> for Breadcrumbs {
    fn from(value: Vec<String>) -> Self {
        Self(value)
    }
}

#[component]
pub fn Header() -> Element {
    let api = use_coroutine_handle::<Api>();
    let mut menu_hidden = use_context::<Signal<MenuHidden>>();
    let breadcrumbs = use_context::<Signal<Breadcrumbs>>();

    rsx!(
        header { class: "p-3",
            div { class: "flex flex-row p-3 space-x-6 items-center",
                img { src: "rustacean-orig-noshadow.svg", width: 50, alt:"",
                    onclick: move|_| menu_hidden.write().toggle(),
                }
                div { class: "text-sm breadcrumbs",
                    ul {
                        li {
                            Link{
                                onclick: |_| (),
                                to: Route::Home {},
                                "首页"
                            }
                        }
                        for path in breadcrumbs.read().0.iter(){
                            li { "{path}" }
                        }
                    }
                }
                button{ class: "absolute right-6 btn",
                    onclick: move |_| api.send(Api::Logout),
                    svg{ xmlns: "http://www.w3.org/2000/svg",
                        view_box: "0 0 24 24",
                        stroke: "currentColor",
                        fill: "none",
                        width: "24",
                        stroke_linejoin: "round",
                        stroke_linecap: "round",
                        height: "24",
                        stroke_width: "2",
                        path { d: "M9 21H5a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h4" },
                        polyline { points: "16 17 21 12 16 7" },
                        line { x1: "21", y1: "12", x2: "9", y2: "12" }
                    }
                }
            }
        }
    )
}
