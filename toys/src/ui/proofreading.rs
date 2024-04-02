use dioxus::prelude::*;

#[derive(Default)]
pub struct InputText(String);

pub fn Proofreading() -> Element {
    let mut input_text = use_context_provider(|| Signal::new(InputText::default()));

    rsx!(article { class:"grid grid-cols-4 gap-3 w-full",
        div{ class:"col-start-1 col-end-4 flex flex-col p-3 border-e",
            label { class: "block text-sm font-medium text-gray-700",
                r#for: "InputText",
                "原文"
            }
            textarea { class: "flex-1 resize-none mt-2 mr-3 p-3 rounded-lg align-top shadow-sm border focus:outline-none sm:text-sm",
                id: "InputText",
                placeholder: "输入需要校验的文本",
                value: &*input_text.read().0,
                onchange: move |evt|{
                    input_text.write().0 = evt.value();
                },
            }
        }
        div{ class:"col-start-4 col-end-5 p-3",
            a { class: "group inline-block rounded w-full bg-gradient-to-r from-pink-500 via-red-500 to-yellow-500 p-[2px] hover:shadow-xl hover:text-white focus:outline-none focus:ring active:text-opacity-75",
                onclick: |_|{
                    //
                },
                span { class: "flex justify-center rounded-sm bg-white px-8 py-3 text-sm font-medium group-hover:bg-transparent",
                    "提交"
                },
            }
        }
    })
}
