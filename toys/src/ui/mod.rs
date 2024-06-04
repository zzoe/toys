use crate::service::Api;
use dioxus::prelude::*;
use dioxus_router::prelude::*;
use strum::{Display, IntoStaticStr};

use crate::ui::config::Settings;
use crate::ui::home::Home;
use crate::ui::proofreading::Proofreading;
use crate::ui::sign::{Sign, AUTHENTICATED};
use crate::ui::sudoku::Sudoku;

pub(crate) mod config;
pub(crate) mod home;
pub(crate) mod proofreading;
pub(crate) mod sign;
mod sudoku;

pub static CURRENT_PAGE: GlobalSignal<CurrentPage> = Signal::global(|| CurrentPage::Home);

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
    #[route("/proofreading")]
    Proofreading {},
    #[route("/settings")]
    Settings {},
}

#[derive(Clone, Display, IntoStaticStr, Eq, PartialEq)]
pub enum CurrentPage {
    #[strum(serialize = "")]
    Home,
    #[strum(serialize = "数独")]
    Sudoku,
    #[strum(serialize = "中文校对-文本纠错")]
    Proofreading,
    #[strum(serialize = "设置")]
    Settings,
}

fn Body() -> Element {
    let mut menu_hidden = use_signal(|| false);
    let menu_hidden_css = menu_hidden().then_some("hidden").unwrap_or_default();
    let nav_hidden_css = CurrentPage::Home
        .eq(&CURRENT_PAGE.read())
        .then_some("hidden")
        .unwrap_or_default();
    let api = use_coroutine_handle::<Api>();

    if AUTHENTICATED() {
        // 登录后
        rsx!(
            div { class: "flex flex-col min-h-screen mx-auto",
            header { class: "p-3",
                div { class: "flex flex-row p-3 space-x-6 items-center",
                    img {
                        width: 50,
                        src: "rustacean-orig-noshadow.svg",
                        alt:"",
                        onclick: move|_| {
                            *menu_hidden.write() = !menu_hidden();
                        }
                    }
                    nav { class:"flex",
                        ol{ class:"flex overflow-hidden rounded-lg border border-gray-200 text-gray-600",
                            li{ class:"flex items-center",
                                Link { class:"flex h-10 items-center gap-1.5 px-4 transition bg-gray-100 hover:text-gray-900",
                                    onclick: |_| {
                                        *CURRENT_PAGE.write() = CurrentPage::Home;
                                    },
                                    to: Route::Home {},
                                    svg{ xmlns:"http://www.w3.org/2000/svg",
                                        class:"h-4 w-4",
                                        fill:"none",
                                        view_box:"0 0 24 24",
                                        stroke:"currentColor",
                                        path{
                                            stroke_linecap:"round",
                                            stroke_linejoin:"round",
                                            stroke_width:"2",
                                            d:"M3 12l2-2m0 0l7-7 7 7M5 10v10a1 1 0 001 1h3m10-11l2 2m-2-2v10a1 1 0 01-1 1h-3m-6 0a1 1 0 001-1v-4a1 1 0 011-1h2a1 1 0 011 1v4a1 1 0 001 1m-6 0h6",
                                        }
                                    },
                                    span{ class:"ms-1.5 text-xs font-medium",
                                        "首页"
                                    }
                                }
                            }
                            li{ class:"relative flex items-center {nav_hidden_css}",
                                span{ class:"absolute inset-y-0 -start-px h-10 w-4 bg-gray-100 [clip-path:_polygon(0_0,_0%_100%,_100%_50%)] rtl:rotate-180"},
                                a{ class:"flex h-10 items-center bg-white pe-4 ps-8 text-xs font-medium transition hover:text-gray-900",
                                    {CURRENT_PAGE.read().to_string()}
                                }
                            }
                        }
                    }
                    button{ class: "absolute right-6 btn",
                        onclick: move |_| {
                            api.send(Api::Logout);
                        },
                        svg {class: "lucide lucide-log-out",
                            view_box: "0 0 24 24",
                            xmlns: "http://www.w3.org/2000/svg",
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
            main { class: "grow flex flex-row p-3 space-x-3",
                nav{ class:"flex flex-col {menu_hidden_css}",
                    ul { class: "menu bg-base-200 w-56 rounded-box",
                        li {
                            details { open: true,
                                summary { "游戏" }
                                ul {
                                    li {
                                        Link { class: "flex flex-row rounded-lg px-4 py-2 text-sm text-nowrap font-medium text-gray-500 hover:bg-gray-100 hover:text-gray-700",
                                            onclick: |_| {
                                                *CURRENT_PAGE.write() = CurrentPage::Sudoku;
                                            },
                                            to: Route::Sudoku {},
                                            svg {
                                                "viewBox": "0 0 24 24",
                                                "xmlns": "http://www.w3.org/2000/svg",
                                                "stroke": "currentColor",
                                                "fill": "none",
                                                width: "24",
                                                "stroke-linejoin": "round",
                                                "stroke-linecap": "round",
                                                height: "24",
                                                "stroke-width": "2",
                                                class: "lucide lucide-grid-3x3",
                                                rect {
                                                    height: "18",
                                                    "x": "3",
                                                    "y": "3",
                                                    "rx": "2",
                                                    width: "18"
                                                }
                                                path { "d": "M3 9h18" }
                                                path { "d": "M3 15h18" }
                                                path { "d": "M9 3v18" }
                                                path { "d": "M15 3v18" }
                                            },
                                            "数独"
                                        }
                                    }
                                }
                            }
                        }
                        li {
                            details { open: true,
                                summary { "中文校对" }
                                ul {
                                    li {
                                        Link { class: "flex flex-row rounded-lg px-4 py-2 text-sm text-nowrap font-medium text-gray-500 hover:bg-gray-100 hover:text-gray-700",
                                            onclick: |_| {
                                                *CURRENT_PAGE.write() = CurrentPage::Proofreading;
                                            },
                                            to: Route::Proofreading {},
                                            svg {
                                                width: "24",
                                                "fill": "none",
                                                "stroke-linecap": "round",
                                                "viewBox": "0 0 24 24",
                                                "stroke-width": "2",
                                                "xmlns": "http://www.w3.org/2000/svg",
                                                "stroke-linejoin": "round",
                                                "stroke": "currentColor",
                                                height: "24",
                                                class: "lucide lucide-spell-check",
                                                path { "d": "m6 16 6-12 6 12" }
                                                path { "d": "M8 12h8" }
                                                path { "d": "m16 20 2 2 4-4" }
                                            }
                                            "文本纠错"
                                        }
                                    }
                                }
                            }
                        }
                        li {
                            details { open: true,
                                summary { "管理" }
                                ul {
                                    li {
                                        Link { class: "flex flex-row rounded-lg px-4 py-2 text-sm text-nowrap font-medium text-gray-500 hover:bg-gray-100 hover:text-gray-700",
                                            onclick: |_| {
                                                *CURRENT_PAGE.write() = CurrentPage::Settings;
                                            },
                                            to: Route::Settings {},
                                            svg { class: "h-5 w-5 mr-3 opacity-75",
                                                xmlns: "http://www.w3.org/2000/svg",
                                                fill: "none",
                                                stroke: "currentColor",
                                                stroke_width: "2",
                                                view_box: "0 0 24 24",
                                                path {
                                                    stroke_linecap: "round",
                                                    stroke_linejoin: "round",
                                                    d: "M10.325 4.317c.426-1.756 2.924-1.756 3.35 0a1.724 1.724 0 002.573 1.066c1.543-.94 3.31.826 2.37 2.37a1.724 1.724 0 001.065 2.572c1.756.426 1.756 2.924 0 3.35a1.724 1.724 0 00-1.066 2.573c.94 1.543-.826 3.31-2.37 2.37a1.724 1.724 0 00-2.572 1.065c-.426 1.756-2.924 1.756-3.35 0a1.724 1.724 0 00-2.573-1.066c-1.543.94-3.31-.826-2.37-2.37a1.724 1.724 0 00-1.065-2.572c-1.756-.426-1.756-2.924 0-3.35a1.724 1.724 0 001.066-2.573c-.94-1.543.826-3.31 2.37-2.37.996.608 2.296.07 2.572-1.065z"
                                                }
                                                path {
                                                    stroke_linejoin: "round",
                                                    d: "M15 12a3 3 0 11-6 0 3 3 0 016 0z",
                                                    stroke_linecap: "round"
                                                }
                                            },
                                            "设置"
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
                Outlet::<Route> {}
            }
            footer { class: "flex flex-row justify-center p-3",
                p { "Copyright © zoe" }
            }
        })
    } else {
        // 未登录
        rsx!(Sign {})
    }
}
