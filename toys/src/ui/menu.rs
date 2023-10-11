use crate::ui::Route;
use dioxus::prelude::*;

pub enum MenUI {
    Title,
    Fold,
    Item,
}

pub struct MenuNode {
    ui_type: MenUI,
    text: String,
    route: Option<Route>,
}

#[derive(Props, PartialEq, Eq)]
pub struct MenuProps {
    pub hidden: bool,
}

pub fn Menu(cx: Scope<MenuProps>) -> Element {
    let nav_hidden = cx.props.hidden.then_some("hidden").unwrap_or_default();
    render!( nav { class: "flex flex-col p-3 space-y-3 min-w-fit {nav_hidden}",
                    label { "1" }
                    label { "2" }
    })
}
