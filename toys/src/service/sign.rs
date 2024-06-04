use reqwest::Method;
use tracing::{error, info};

use toy_schema::sign::SignReq;

use crate::service::http;
use crate::ui::sign::{AlertMsg, AlertType, ALERT_MSG, AUTHENTICATED};

pub async fn sign_up(req: SignReq) {
    if let Err(e) = http::<SignReq, ()>(Method::POST, "/api/sign_up", Some(&req)).await {
        error!("注册失败： {e}");
        *ALERT_MSG.write() = AlertMsg::new(Some(AlertType::Error), "信息有误，注册失败");
    } else {
        *AUTHENTICATED.write() = true;
    }
}

pub async fn sign_in(req: SignReq) {
    if let Err(e) = http::<SignReq, ()>(Method::POST, "/api/sign_in", Some(&req)).await {
        error!("登录失败： {e}");
        *ALERT_MSG.write() = AlertMsg::new(Some(AlertType::Error), "用户名或密码错误");
    } else {
        *AUTHENTICATED.write() = true;
    }
}

pub async fn sign_check() {
    if let Ok(true) = http::<(), bool>(Method::POST, "/api/sign_check", None).await {
        *AUTHENTICATED.write() = true;
    } else {
        *AUTHENTICATED.write() = false;
    }
}

pub async fn logout() {
    *AUTHENTICATED.write() = false;
    if let Err(e) = http::<(), ()>(Method::POST, "/api/logout", None).await {
        error!("登出失败： {e}");
    } else {
        info!("登出成功！")
    }
}
