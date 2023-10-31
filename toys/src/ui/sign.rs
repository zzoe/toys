use crate::service::{login, Api};
use dioxus::prelude::*;

pub fn Sign(cx: Scope) -> Element {
    let sign_in = use_state(cx, || false);
    let user_name = use_state(cx, || "".to_string());
    let user_email = use_state(cx, || "".to_string());
    let user_password = use_state(cx, || "".to_string());

    let api = use_coroutine_handle::<Api>(cx).unwrap();

    render!(
        div { class: "flex flex-col min-h-screen",
            header { class: "p-3",
                div { class: "flex flex-row p-3 space-x-3 items-center",
                    img {
                        width: 50,
                        src: "rustacean-orig-noshadow.svg",
                        alt: "",
                    }
                }
            }
            main { class: "grow flex flex-col p-3 space-y-3 justify-center items-center",
                input{
                    id: "name",
                    r#type: "text",
                    placeholder: "姓名",
                    hidden: *sign_in.get(),
                    onchange: move |evt| user_name.set(evt.value.clone()),
                }
                input{
                    id: "email",
                    r#type: "email",
                    placeholder: "邮箱",
                    onchange: move |evt| user_email.set(evt.value.clone()),
                }
                input{
                    id: "password",
                    r#type: "password",
                    placeholder: "密码",
                    onchange: move |evt| user_password.set(evt.value.clone()),
                }
                div { class: "flex flex-row p-3 space-x-3 items-center",
                    input{
                        id: "button",
                        r#type: "button",
                        value: "登录",
                        onclick: |_|api.send(Api::Login(login::Req{
                            name: user_name.to_string(),
                            email: user_email.to_string(),
                            password: user_password.to_string(),
                        })),
                    }
                }
            }
            footer { class: "flex flex-row justify-center p-3", p { "Copyright © zoe" } }
        }
    )
}
