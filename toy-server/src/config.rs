use std::io::ErrorKind::NotFound;
use std::sync::{Arc, OnceLock};
use std::{env, fs};

use arc_swap::ArcSwap;
use serde::{Deserialize, Serialize};

pub(crate) static GLOBAL_CONFIG: OnceLock<ArcSwap<Config>> = OnceLock::new();

/// 加载配置
pub(crate) fn reload() {
    let file = env::var("APP_ENV")
        .map(|e| format!("config.{e}.toml"))
        .unwrap_or("config.prd.toml".into());
    let cfg = GLOBAL_CONFIG.get_or_init(|| ArcSwap::new(Arc::new(Config::default())));

    match fs::read_to_string(&file) {
        Ok(s) => match toml::from_str::<Config>(&s) {
            Ok(c) => {
                cfg.store(Arc::new(c));
            }
            Err(e) => log::error!("{file} 配置格式有误，解析失败: {e}"),
        },
        Err(e) => {
            if e.kind() == NotFound {
                if let Ok(c) = toml::to_string(&Config::default()) {
                    fs::write(file, c).ok();
                }
            } else {
                log::error!("{file} 配置读取失败: {e}");
            }
        }
    }
}

#[derive(Deserialize, Serialize, Default)]
pub(crate) struct Config {
    pub(crate) log: LogCfg,
    pub(crate) web: WebCfg,
    pub(crate) trace: TraceCfg,
}

#[derive(Deserialize, Serialize)]
pub(crate) struct LogCfg {
    pub(crate) directory: String,
    pub(crate) level: String,
}

impl Default for LogCfg {
    fn default() -> Self {
        LogCfg {
            directory: "../logs".to_owned(),
            level: "INFO".to_owned(),
        }
    }
}

#[derive(Deserialize, Serialize)]
pub(crate) struct WebCfg {
    pub(crate) address: String,
    pub(crate) assets_path: String,
}

impl Default for WebCfg {
    fn default() -> Self {
        WebCfg {
            address: "0.0.0.0:8080".to_owned(),
            assets_path: "../dist/public".to_owned(),
        }
    }
}

#[derive(Deserialize, Serialize)]
pub(crate) struct TraceCfg {
    pub(crate) authorization: String,
    pub(crate) assets_path: String,
}

impl Default for TraceCfg {
    fn default() -> Self {
        TraceCfg {
            authorization: "0.0.0.0:8080".to_owned(),
            assets_path: "./dist".to_owned(),
        }
    }
}