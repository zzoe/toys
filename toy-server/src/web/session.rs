use std::collections::BTreeMap;
use std::future::Future;
use std::time::Duration;

use poem::session::SessionStorage;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use surrealdb::engine::remote::ws::Client;
use surrealdb::Surreal;
use tracing::{debug, error, info};

use crate::error::Error;
use crate::web::database;
use crate::web::database::ROOT_CREDENTIALS;

pub struct SurrealStorage {
    db: Surreal<Client>,
}

// 直接用BTreeMap<String,Value>会报错：
// Serialization error: invalid type: enum, expected any valid JSON value
// 所以用这个结构体来中转一下
#[derive(Deserialize, Serialize)]
struct SessionMap {
    session: String,
}

impl SurrealStorage {
    pub(crate) async fn new() -> Result<Self, surrealdb::Error> {
        let db = database::connect().await?;
        db.signin(ROOT_CREDENTIALS).await?;
        Ok(SurrealStorage { db })
    }
}

impl SessionStorage for SurrealStorage {
    fn load_session<'a>(
        &'a self,
        session_id: &'a str,
    ) -> impl Future<Output = poem::Result<Option<BTreeMap<String, Value>>>> + Send + 'a {
        info!("load session {session_id}");
        async move {
            match self.db.select(("session", session_id)).await {
                Ok(Some::<SessionMap>(s)) => {
                    let session: BTreeMap<String, Value> =
                        serde_json::from_str(&s.session).unwrap();
                    Ok(Some(session))
                }
                Ok(None) => Ok(None),
                Err(e) => {
                    error!("select session 失败：{e}");
                    Err(Error::DbException(e).into())
                }
            }
        }
    }

    fn update_session<'a>(
        &'a self,
        session_id: &'a str,
        entries: &'a BTreeMap<String, Value>,
        _expires: Option<Duration>,
    ) -> impl Future<Output = poem::Result<()>> + Send + 'a {
        let session = serde_json::to_string(entries).unwrap();
        info!("upsert session {session_id}: {session}");
        async move {
            self.db
                .upsert::<Option<SessionMap>>(("session", session_id))
                .content(SessionMap { session })
                .await
                .map(|_| ())
                .map_err(|e| {
                    error!("upsert session 失败：{e}");
                    Error::DbException(e).into()
                })
        }
    }

    fn remove_session<'a>(
        &'a self,
        session_id: &'a str,
    ) -> impl Future<Output = poem::Result<()>> + Send + 'a {
        debug!("remove session {session_id}");
        async move {
            self.db
                .delete::<Option<SessionMap>>(("session", session_id))
                .await
                .map(|_| ())
                .map_err(|e| {
                    error!("delete session 失败：{e}");
                    Error::DbException(e).into()
                })
        }
    }
}
