use dioxus::prelude::*;
use dioxus_router::prelude::*;
use fermi::{use_atom_state, use_read, Atom, AtomId};
use strum::{Display, IntoStaticStr};

use crate::ui::config::Settings;
use crate::ui::home::Home;
use crate::ui::proofreading::Proofreading;
use crate::ui::sign::{Sign, AUTHENTICATED};

pub(crate) mod config;
pub(crate) mod home;
pub(crate) mod proofreading;
pub(crate) mod sign;

pub static CURRENT_PAGE: Atom<CurrentPage> = Atom(|_| CurrentPage::Home);

pub fn unique_id<V>(atom: &'static Atom<V>) -> AtomId {
    atom as *const Atom<V> as AtomId
}

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

#[derive(Display, IntoStaticStr, Eq, PartialEq)]
pub enum CurrentPage {
    #[strum(serialize = "")]
    Home,
    #[strum(serialize = "中文校对-文本纠错")]
    Proofreading,
    #[strum(serialize = "设置")]
    Settings,
}

fn Body(cx: Scope) -> Element {
    let menu_hidden = use_state(cx, || false);
    let menu_hidden_css = menu_hidden.get().then_some("hidden").unwrap_or_default();
    let authenticated = use_read(cx, &AUTHENTICATED);
    let current_page = use_atom_state(cx, &CURRENT_PAGE);
    let current_page_str: &'static str = current_page.get().into();
    let nav_hidden_css = CurrentPage::Home
        .eq(current_page.get())
        .then_some("hidden")
        .unwrap_or_default();

    if *authenticated {
        // 登录后
        render!(
            div { class: "flex flex-col min-h-screen",
            header { class: "p-3",
                div { class: "flex flex-row p-3 space-x-6 items-center",
                    img {
                        width: 50,
                        src: "rustacean-orig-noshadow.svg",
                        alt:"",
                        onclick: |_| {
                            menu_hidden.set(!*menu_hidden.get());
                        }
                    }
                    nav { class:"flex",
                        ol{ class:"flex overflow-hidden rounded-lg border border-gray-200 text-gray-600",
                            li{ class:"flex items-center",
                                Link { class:"flex h-10 items-center gap-1.5 px-4 transition bg-gray-100 hover:text-gray-900",
                                    onclick: |_| {
                                        current_page.set(CurrentPage::Home);
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
                                        match current_page.get(){
                                            CurrentPage::Home => Route::Home {},
                                            CurrentPage::Settings => Route::Settings {},
                                            _ => Route::Proofreading {}
                                        }
                                    },
                                    current_page_str
                                }
                            }
                        }
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
                                                    current_page.set(CurrentPage::Proofreading);
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
                                    active_class: if CurrentPage::Settings.eq(current_page.get()) {"bg-gray-100 text-gray-700"} else {""},
                                    onclick: |_| {
                                        current_page.set(CurrentPage::Settings);
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
        render!(Sign {})
    }
}
