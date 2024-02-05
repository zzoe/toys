use std::rc::Rc;

use fermi::AtomRoot;
use reqwest::Method;
use tracing::error;

use toy_schema::sign::{SignCheck, SignReq};

use crate::service::http;
use crate::ui::sign::{ALERT_MSG, AlertMsg, AlertType, AUTHENTICATED};
use crate::ui::unique_id;

pub async fn sign_up(atoms: Rc<AtomRoot>, req: SignReq) {
    if let Err(e) = http(Method::POST, "/sign_up", &req).await {
        error!("注册失败： {e}");
        atoms.set(
            unique_id(&ALERT_MSG),
            AlertMsg::new(Some(AlertType::Error), "信息有误，注册失败"),
        );
    } else {
        atoms.set(unique_id(&AUTHENTICATED), true);
    }
}

pub async fn sign_in(atoms: Rc<AtomRoot>, req: SignReq) {
    if let Err(e) = http(Method::POST, "/sign_in", &req).await {
        error!("登录失败： {e}");
        atoms.set(
            unique_id(&ALERT_MSG),
            AlertMsg::new(Some(AlertType::Error), "用户名或密码错误"),
        );
    } else {
        atoms.set(unique_id(&AUTHENTICATED), true);
    }
}

pub async fn sign_check(atoms: Rc<AtomRoot>) {
    if http(Method::POST, "/sign_check", &SignCheck {})
        .await
        .is_ok()
    {
        atoms.set(unique_id(&AUTHENTICATED), true);
    }
}
