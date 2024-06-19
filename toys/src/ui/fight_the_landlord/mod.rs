use dioxus::prelude::*;

#[component]
pub fn FightTheLandlord() -> Element {
    rsx!(
        div{ class:"flex flex-col",
            div{ class:"flex flex-row gap-3",
                select { class: "select select-sm w-full max-w-xs",
                    option { selected: "true", "AI辅助" }
                }
                button{ class:"btn",
                    "初始化"
                }
                button{ class:"btn",
                    "开始"
                }
            }

        }
    )
}
