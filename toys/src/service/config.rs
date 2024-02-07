use fermi::AtomRoot;
use reqwest::Method;
use tracing::{error, info};

use crate::service::http;
use crate::ui::config::SETTINGS_BTN_DISABLE;
use crate::ui::unique_id;

pub async fn reload(atoms: &AtomRoot) {
    if let Err(e) = http::<()>(atoms, Method::POST, "/api/auth/reload", None).await {
        error!("更新配置失败：{e}");
    } else {
        info!("配置加载成功");
    }
    atoms.set(unique_id(&SETTINGS_BTN_DISABLE), false);
}
