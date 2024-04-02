use reqwest::Method;
use tracing::error;

use toy_schema::sign::SignReq;

use crate::service::http;
use crate::ui::sign::{ALERT_MSG, AlertMsg, AlertType, AUTHENTICATED};

pub async fn sign_up(req: SignReq) {
    if let Err(e) = http(Method::POST, "/api/sign_up", Some(&req)).await {
        error!("注册失败： {e}");
        *ALERT_MSG.write() = AlertMsg::new(Some(AlertType::Error), "信息有误，注册失败");
    } else {
        *AUTHENTICATED.write() = true;
    }
}

pub async fn sign_in(req: SignReq) {
    if let Err(e) = http(Method::POST, "/api/sign_in", Some(&req)).await {
        error!("登录失败： {e}");
        *ALERT_MSG.write() = AlertMsg::new(Some(AlertType::Error), "用户名或密码错误");
    } else {
        *AUTHENTICATED.write() = true;
    }
}

pub async fn sign_check() {
    if http::<()>(Method::POST, "/api/sign_check", None)
        .await
        .is_ok()
    {
        *AUTHENTICATED.write() = true;
    }
}
