use reqwest::Method;
use tracing::{error, info};

use crate::service::http;
use crate::ui::config::SETTINGS_BTN_DISABLE;

pub async fn reload() {
    if let Err(e) = http::<(), ()>(Method::POST, "/api/reload", None).await {
        error!("更新配置失败：{e}");
    } else {
        info!("配置加载成功");
    }
    *SETTINGS_BTN_DISABLE.write() = false;
}
