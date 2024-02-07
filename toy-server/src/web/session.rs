use std::collections::BTreeMap;
use std::time::Duration;

use poem::async_trait;
use poem::session::SessionStorage;
use serde_json::Value;
use surrealdb::engine::remote::ws::Client;
use surrealdb::Surreal;
use tracing::{debug, info};

use crate::error::ErrorConv;
use crate::web::database;
use crate::web::database::ROOT_CREDENTIALS;

pub struct SurrealStorage {
    db: Surreal<Client>,
}

impl SurrealStorage {
    pub(crate) async fn new() -> Result<Self, surrealdb::Error> {
        let db = database::connect().await?;
        db.signin(ROOT_CREDENTIALS).await?;

        // Define the table and scope
        // db.query(r#"DEFINE TABLE user SCHEMAFULL
        //         PERMISSIONS
        //             FOR select, update, delete WHERE id = $auth.id"#)
        //     .query(r#"DEFINE FIELD name ON user TYPE string"#)
        //     .query(r#"DEFINE FIELD email ON user TYPE string ASSERT string::is::email($value)"#)
        //     .query(r#"DEFINE FIELD password ON user TYPE string"#)
        //     .query(r#"DEFINE INDEX email ON user FIELDS email UNIQUE"#)
        //     .query(r#"DEFINE SCOPE user_scope SESSION 10h
        //     SIGNUP ( CREATE user CONTENT {
        //         name: $name,
        //         email: $email,
        //         password: crypto::argon2::generate($password)
        //     })
        //     SIGNIN ( SELECT * FROM user WHERE email = $email AND crypto::argon2::compare(password, $password) )"#)
        // .await?
        // .check()?;
        // debug!("table init complete");

        Ok(SurrealStorage { db })
    }
}

#[async_trait]
impl SessionStorage for SurrealStorage {
    async fn load_session(
        &self,
        session_id: &str,
    ) -> poem::Result<Option<BTreeMap<String, Value>>> {
        info!("load session {session_id}");
        match self
            .db
            .select(("session", session_id))
            // .query("select * omit id from session where id = $id")
            // .bind(("id", format!("session:{session_id}")))
            .await
        {
            Ok(Some::<BTreeMap<String, Value>>(mut res)) => {
                res.remove("id");
                Ok(Some(res))
            }
            o => o.internal_server_error(),
        }
    }

    async fn update_session(
        &self,
        session_id: &str,
        entries: &BTreeMap<String, Value>,
        _expires: Option<Duration>,
    ) -> poem::Result<()> {
        info!("update session {session_id}");
        info!("{entries:#?}");
        self.db
            .update::<Option<BTreeMap<String, Value>>>(("session", session_id))
            .content(entries)
            .await
            .map(|_| ())
            .internal_server_error()
    }

    async fn remove_session(&self, session_id: &str) -> poem::Result<()> {
        debug!("remove session {session_id}");
        self.db
            .delete::<Option<BTreeMap<String, Value>>>(("session", session_id))
            .await
            .map(|_| ())
            .internal_server_error()
    }
}
