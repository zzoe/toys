use dioxus::core_macro::component;
use dioxus::dioxus_core::Element;
use dioxus::prelude::*;
use dioxus_router::prelude::*;

use crate::ui::header::Breadcrumbs;
use crate::ui::route::Route;

pub struct MenuHidden(pub bool);

impl MenuHidden {
    pub fn toggle(&mut self) {
        self.0 = !self.0
    }
}

#[component]
pub fn Menu() -> Element {
    let mut breadcrumbs = use_context::<Signal<Breadcrumbs>>();
    let menu_hidden = use_context::<Signal<MenuHidden>>()
        .read()
        .0
        .then_some("hidden")
        .unwrap_or_default();

    rsx!(
        nav{ class: "flex flex-col {menu_hidden}",
            ul { class: "menu bg-base-200 w-56 rounded-box",
                li {
                    details { open: true,
                        summary { "游戏" }
                        ul {
                            li {
                                Link { class: "flex flex-row rounded-lg px-4 py-2 text-sm text-nowrap font-medium text-gray-500 hover:bg-gray-100 hover:text-gray-700",
                                    onclick: move|_| breadcrumbs.set(vec!["游戏".into(), "数独".into()].into()),
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
                            li {
                                Link { class: "flex flex-row rounded-lg px-4 py-2 text-sm text-nowrap font-medium text-gray-500 hover:bg-gray-100 hover:text-gray-700",
                                    onclick: move|_| breadcrumbs.set(vec!["游戏".into(), "斗地主".into()].into()),
                                    to: Route::FightTheLandlord {},
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
                                        path { "d": "M5 9c-1.5 1.5-3 3.2-3 5.5A5.5 5.5 0 0 0 7.5 20c1.8 0 3-.5 4.5-2 1.5 1.5 2.7 2 4.5 2a5.5 5.5 0 0 0 5.5-5.5c0-2.3-1.5-4-3-5.5l-7-7-7 7Z" }
                                        path { "d": "M12 18v4" }
                                    },
                                    "斗地主"
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
                                    onclick: move|_| breadcrumbs.set(vec!["中文校对".into(), "文本纠错".into()].into()),
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
                                    onclick: move|_| breadcrumbs.set(vec!["管理".into(), "设置".into()].into()),
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
    )
}
