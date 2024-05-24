use dioxus::prelude::*;

use toy_schema::sign::SignReq;
use tracing::info;

use crate::service::Api;

pub static AUTHENTICATED: GlobalSignal<bool> = Signal::global(|| false);
pub static ALERT_MSG: GlobalSignal<AlertMsg> = Signal::global(Default::default);

#[derive(Default)]
pub struct AlertMsg {
    pub typ: Option<AlertType>,
    pub msg: String,
}

impl AlertMsg {
    pub fn new(typ: Option<AlertType>, msg: impl ToString) -> Self {
        AlertMsg {
            typ,
            msg: msg.to_string(),
        }
    }
}

pub enum AlertType {
    Info,
    Warn,
    Error,
}

pub fn Sign() -> Element {
    let mut sign_in = use_signal(|| true);
    let mut user_name = use_signal(|| "".to_string());
    let mut user_email = use_signal(|| "".to_string());
    let mut user_password = use_signal(|| "".to_string());
    let api = use_coroutine_handle::<Api>();
    api.send(Api::SignCheck);

    rsx!(
        div { class: "text-gray-800 antialiased",
            nav{class:"top-0 absolute z-50 w-full flex flex-wrap items-center justify-between px-2 py-3 ",
                div{ class:"container px-4 mx-auto flex flex-wrap items-center justify-between",
                    div{class:"w-full relative flex justify-between lg:w-auto lg:static lg:block lg:justify-start",
                        img {
                            width: 50,
                            src: "rustacean-orig-noshadow.svg",
                            alt: "",
                        }
                    }
                }
            }
            main{
                section{ class: "absolute w-full h-full",
                    div{class:"absolute top-0 w-full h-full bg-gray-900",
                        style: "background-image: url(register_bg.png); background-size: 100%; background-repeat: no-repeat;"
                    }
                    div{class: "mx-auto sm:w-3/4 md:w-2/4 fixed z-50 inset-x-0 top-10 rounded-xl border border-gray-100 bg-white p-4",
                        role: "alert",
                        hidden: ALERT_MSG.read().typ.is_none(),
                        div{class: "flex items-start gap-4 text-red-600",
                            img{width: 24,src:"error.svg",alt:""}
                            div{class:"flex-1",
                                strong{class:"block font-medium", "失败"}
                                p{class:"mt-1 text-sm", {ALERT_MSG.read().msg.as_str()}}
                            }
                            button{class:"text-gray-500 transition hover:text-gray-600",
                                onclick: |_| *ALERT_MSG.write() = Default::default(),
                                svg{xmlns:"http://www.w3.org/2000/svg",
                                    fill: "none",
                                    view_box: "0 0 24 24",
                                    stroke_width: "1.5",
                                    stroke:"currentColor",
                                    class:"h-6 w-6",
                                    path{stroke_linecap:"round",
                                        stroke_linejoin:"round",
                                        d:"M6 18L18 6M6 6l12 12"
                                    }
                                }
                            }
                        }
                    }
                    div{ class:"container mx-auto px-4 h-full",
                        div{ class: "flex content-center items-center justify-center h-full",
                            div{ class:"w-full lg:w-6/12 px-4",
                                div{ class:"relative flex flex-col min-w-0 break-words w-full mb-6 shadow-lg rounded-lg bg-gray-300 border-0",
                                    div{class:"flex-auto px-4 py-6 lg:px-10",
                                        // form{
                                            div{class:"relative w-full mb-3",
                                                hidden: *sign_in.read(),
                                                label{class:"block text-gray-700 text-xs font-bold mb-2",
                                                    r#for: "grid-password",
                                                    "姓名",
                                                }
                                                input{ class: "border-0 px-3 py-3 placeholder-gray-400 text-gray-700 bg-white rounded text-sm shadow focus:outline-none focus:ring w-full",
                                                    id: "name",
                                                    r#type: "text",
                                                    placeholder: "姓名",
                                                    style:"transition: all 0.15s ease 0s;",
                                                    onchange: move |evt| user_name.set(evt.value()),
                                                }
                                            }
                                            div{class:"relative w-full mb-3",
                                                label{class:"block text-gray-700 text-xs font-bold mb-2",
                                                    r#for: "grid-password",
                                                    "邮箱",
                                                }
                                                input{ class: "border-0 px-3 py-3 placeholder-gray-400 text-gray-700 bg-white rounded text-sm shadow focus:outline-none focus:ring w-full",
                                                    id: "email",
                                                    r#type: "email",
                                                    placeholder: "邮箱",
                                                    style:"transition: all 0.15s ease 0s;",
                                                    onchange: move |evt| user_email.set(evt.value()),
                                                }
                                            }
                                            div{class:"relative w-full mb-3",
                                                label{class:"block text-gray-700 text-xs font-bold mb-2",
                                                    r#for: "grid-password",
                                                    "密码",
                                                }
                                                input{ class: "border-0 px-3 py-3 placeholder-gray-400 text-gray-700 bg-white rounded text-sm shadow focus:outline-none focus:ring w-full",
                                                    id: "password",
                                                    r#type: "password",
                                                    placeholder: "密码",
                                                    autocomplete: "off",
                                                    style:"transition: all 0.15s ease 0s;",
                                                    onchange: move |evt| user_password.set(evt.value()),
                                                }
                                            }
                                            div{class:"text-center mt-6 mb-3",
                                                button{ class:"btn btn-primary btn-wide",
                                                // class: "bg-gray-900 text-white active:bg-gray-700 text-sm font-bold px-6 py-3 rounded shadow hover:shadow-lg outline-none focus:outline-none mr-1 mb-1 w-full",
                                                    style: "transition: all 0.15s ease 0s;",
                                                    id: "button-submit",
                                                    // r#type: "submit",
                                                    onclick: move |_|{
                                                        info!("登录/注册");
                                                        if *sign_in.read(){
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
                                                    if *sign_in.read(){
                                                        "登录"
                                                    }else {
                                                        "注册"
                                                    },
                                                }
                                            }
                                            div{class:"flex flex-wrap",
                                                a{ class: "text-xs cursor-pointer",
                                                    style: "color: lightslategray;",
                                                    onclick: move |_| *sign_in.write() = !sign_in(),
                                                    if !*sign_in.read(){
                                                        "已有账号，立即登录"
                                                    }else {
                                                        "尚无账号，立即注册"
                                                    },
                                                }
                                            }
                                        // }
                                    }
                                }
                            }
                        }
                    }
                }
            }
            footer { class: "absolute w-full bottom-0 bg-gray-900 pb-6",
                div{class:"container mx-auto px-4",
                    hr{class:"mb-6 border-b-1 border-gray-700"}
                    div{class:"flex flex-wrap items-center justify-center",
                        div{class:"text-sm text-white font-semibold py-1",
                            "Copyright © zoe"
                        }
                    }
                }
            }
        }
    )
}
