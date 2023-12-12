use dioxus::prelude::*;

use toy_schema::sign::SignReq;

use crate::service::Api;

pub fn Sign(cx: Scope) -> Element {
    let sign_in = use_state(cx, || true);
    let user_name = use_state(cx, || "".to_string());
    let user_email = use_state(cx, || "".to_string());
    let user_password = use_state(cx, || "".to_string());

    let api = use_coroutine_handle::<Api>(cx).unwrap();
    api.send(Api::SignCheck);

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
                    class: "border-2 rounded",
                    id: "name",
                    r#type: "text",
                    placeholder: "姓名",
                    hidden: *sign_in.get(),
                    onchange: move |evt| user_name.set(evt.value.clone()),
                }
                input{
                    class: "border-2 rounded",
                    id: "email",
                    r#type: "email",
                    placeholder: "邮箱",
                    onchange: move |evt| user_email.set(evt.value.clone()),
                }
                input{
                    class: "border-2 rounded",
                    id: "password",
                    r#type: "password",
                    placeholder: "密码",
                    onchange: move |evt| user_password.set(evt.value.clone()),
                }
                input{
                    class: "border-2 rounded",
                    id: "button-submit",
                    r#type: "button",
                    value: if *sign_in.get(){
                        "登录"
                    }else {
                        "注册"
                    },
                    onclick: |_|{
                        if *sign_in.get(){
                            api.send(Api::SignIn(SignReq{
                                name: user_name.to_string(),
                                email: user_email.to_string(),
                                password: user_password.to_string(),
                            }));
                        }else{
                            api.send(Api::SignUp(SignReq{
                                name: user_name.to_string(),
                                email: user_email.to_string(),
                                password: user_password.to_string(),
                            }));
                        }
                    },
                }
                input{
                    class: "border-2 rounded text-xs",
                    id: "button-conv",
                    r#type: "button",
                    value: if !*sign_in.get(){
                        "已有账号，立即登录"
                    }else {
                        "尚无账号，立即注册"
                    },
                    onclick: |_| sign_in.set(!*sign_in.get()),
                }
            }
            footer { class: "flex flex-row justify-center p-3", p { "Copyright © zoe" } }
        }
    )
}
