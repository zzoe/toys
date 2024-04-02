use dioxus::prelude::*;
use dioxus_router::prelude::*;
use strum::{Display, IntoStaticStr};

use crate::ui::config::Settings;
use crate::ui::home::Home;
use crate::ui::proofreading::Proofreading;
use crate::ui::sign::{Sign, AUTHENTICATED};

pub(crate) mod config;
pub(crate) mod home;
pub(crate) mod proofreading;
pub(crate) mod sign;

pub static CURRENT_PAGE: GlobalSignal<CurrentPage> = Signal::global(|| CurrentPage::Home);

#[rustfmt::skip]
#[derive(Clone, Debug, PartialEq, Routable)]
pub enum Route {
  #[layout(Body)]
    #[route("/")]
    Home {},
    #[route("/proofreading")]
    Proofreading {},
    #[route("/settings")]
    Settings {},
}

#[derive(Clone, Display, IntoStaticStr, Eq, PartialEq)]
pub enum CurrentPage {
    #[strum(serialize = "")]
    Home,
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
                                Link{ class:"flex h-10 items-center bg-white pe-4 ps-8 text-xs font-medium transition hover:text-gray-900",
                                    to: {
                                        match *CURRENT_PAGE.read(){
                                            CurrentPage::Home => Route::Home {},
                                            CurrentPage::Settings => Route::Settings {},
                                            _ => Route::Proofreading {}
                                        }
                                    },
                                    {CURRENT_PAGE.read().to_string()}
                                }
                            }
                        }
                    }
                    svg {class: "absolute right-6 lucide lucide-log-out",
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
            main { class: "grow flex flex-row p-3 space-x-3",
                nav{ class:"flex flex-row",
                    div { class: "flex flex-col w-40 justify-between border-e bg-white {menu_hidden_css}",
                        ul { class: "space-y-1",
                            li {
                                details { class: "group [&_summary::-webkit-details-marker]:hidden",
                                    open: true,
                                    summary { class: "flex cursor-pointer items-center justify-between rounded-lg px-4 py-2 text-gray-500 hover:bg-gray-100 hover:text-gray-700",
                                        span { class: "flex flex-row text-sm text-nowrap font-medium",
                                            // img{ class:"h-5 w-5 mr-3 opacity-75",
                                            //     src:"xunfei.ico",
                                            //     alt:"",
                                            // },
                                            "中文校对",
                                        }
                                        span { class: "shrink-0 transition duration-300 group-open:-rotate-180",
                                            svg { class: "h-5 w-5",
                                                view_box: "0 0 20 20",
                                                fill: "currentColor",
                                                xmlns: "http://www.w3.org/2000/svg",
                                                path {
                                                    fill_rule: "evenodd",
                                                    d: "M5.293 7.293a1 1 0 011.414 0L10 10.586l3.293-3.293a1 1 0 111.414 1.414l-4 4a1 1 0 01-1.414 0l-4-4a1 1 0 010-1.414z",
                                                    clip_rule: "evenodd"
                                                }
                                            }
                                        }
                                    }
                                    ul { class: "mt-2 space-y-1 px-4",
                                        li {
                                            Link { class: "block rounded-lg px-4 py-2 text-sm text-nowrap font-medium text-gray-500 hover:bg-gray-100 hover:text-gray-700",
                                                onclick: |_| {
                                                    *CURRENT_PAGE.write() = CurrentPage::Proofreading;
                                                },
                                                to: Route::Proofreading {},
                                                "文本纠错"
                                            }
                                        }
                                    }
                                }
                            }
                            li { class: "bottom-0",
                                Link { class: "flex flex-row rounded-lg px-4 py-2 text-sm text-nowrap font-medium text-gray-500 hover:bg-gray-100 hover:text-gray-700",
                                    active_class: if CurrentPage::Settings.eq(&CURRENT_PAGE.read()) {"bg-gray-100 text-gray-700"} else {""},
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
