use fermi::AtomRoot;
use reqwest::Method;
use tracing::error;

use toy_schema::sign::SignReq;

use crate::service::http;
use crate::ui::sign::{AlertMsg, AlertType, ALERT_MSG, AUTHENTICATED};
use crate::ui::unique_id;

pub async fn sign_up(atoms: &AtomRoot, req: SignReq) {
    if let Err(e) = http(atoms, Method::POST, "/api/sign_up", Some(&req)).await {
        error!("注册失败： {e}");
        atoms.set(
            unique_id(&ALERT_MSG),
            AlertMsg::new(Some(AlertType::Error), "信息有误，注册失败"),
        );
    } else {
        atoms.set(unique_id(&AUTHENTICATED), true);
    }
}

pub async fn sign_in(atoms: &AtomRoot, req: SignReq) {
    if let Err(e) = http(atoms, Method::POST, "/api/sign_in", Some(&req)).await {
        error!("登录失败： {e}");
        atoms.set(
            unique_id(&ALERT_MSG),
            AlertMsg::new(Some(AlertType::Error), "用户名或密码错误"),
        );
    } else {
        atoms.set(unique_id(&AUTHENTICATED), true);
    }
}

pub async fn sign_check(atoms: &AtomRoot) {
    if http::<()>(atoms, Method::POST, "/api/sign_check", None)
        .await
        .is_ok()
    {
        atoms.set(unique_id(&AUTHENTICATED), true);
    }
}
