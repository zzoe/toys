use dioxus::prelude::*;
use fermi::{Atom, use_atom_state};
use tracing::info;

use crate::service::Api;

pub static SETTINGS_BTN_DISABLE: Atom<bool> = Atom(|_| false);

pub fn Settings(cx: Scope) -> Element {
    let api = use_coroutine_handle::<Api>(cx).unwrap();
    let settings_btn_disable = use_atom_state(cx, &SETTINGS_BTN_DISABLE);

    render!(article { class:"flex flex-col p-3 space-y-3",
        h1{
            "Settings"
        }
        button { class: "group inline-block rounded w-full enabled:outline outline-red-500 disabled:bg-gray-300
        enabled:hover:bg-gradient-to-r enabled:hover:from-pink-500 enabled:hover:via-red-500 enabled:hover:to-yellow-500
        hover:outline-none hover:shadow-xl hover:text-white active:text-opacity-75",
            r#type: "button",
            disabled: *settings_btn_disable.get(),
            onclick: |_|{
                info!("begin config reload");
                settings_btn_disable.set(true);
                api.send(Api::ConfigReload);
            },
            span { class: "flex justify-center rounded-sm bg-white px-8 py-3 text-sm font-medium group-hover:bg-transparent
            group-disabled:bg-transparent",
                "提交"
            }
        }
    })
}
