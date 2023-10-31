use std::env::current_dir;
use std::path::PathBuf;

use arc_swap::access::Access;
use surrealdb::engine::local::{Db, RocksDb};
use surrealdb::opt::auth::Root;
use surrealdb::opt::Config as DBConfig;
use surrealdb::Surreal;

use crate::config::{Config, GLOBAL_CONFIG};
use crate::error::Error;

pub(crate) const ROOT_CREDENTIALS: Root = Root {
    username: "root",
    password: "root_pass",
};

pub(crate) async fn new() -> Result<Surreal<Db>, Error> {
    let db = Surreal::new::<RocksDb>(endpoints(ROOT_CREDENTIALS)).await?;

    db.use_ns("toy").use_db("toy").await?;

    Ok(db)
}

fn current_relative_path() -> PathBuf {
    let Ok(current_path) = current_dir() else {
        return PathBuf::new();
    };
    let Some(prefix) = current_path.ancestors().last() else {
        return current_path;
    };
    let Ok(path) = current_path.strip_prefix(prefix) else {
        return current_path;
    };
    path.to_path_buf()
}

fn endpoints(credentials: Root) -> (PathBuf, DBConfig) {
    let cfg = GLOBAL_CONFIG
        .get()
        .unwrap()
        .map(|cfg: &Config| &cfg.web)
        .load();

    let path = current_relative_path().join(&cfg.db_path);
    let cfg = DBConfig::new().user(credentials);

    (path, cfg)
}
