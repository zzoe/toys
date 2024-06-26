use std::collections::BTreeMap;
use std::future::Future;
use std::time::Duration;

use poem::session::SessionStorage;
use serde_json::Value;
use surrealdb::engine::remote::ws::Client;
use surrealdb::Surreal;
use tracing::info;

use crate::error::Error;
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

impl SessionStorage for SurrealStorage {
    fn load_session<'a>(
        &'a self,
        session_id: &'a str,
    ) -> impl Future<Output = poem::Result<Option<BTreeMap<String, Value>>>> + Send + 'a {
        async move {
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
                Ok(None) => Ok(None),
                Err(e) => Err(Error::DbException(e).into()),
            }
        }
    }

    fn update_session<'a>(
        &'a self,
        session_id: &'a str,
        entries: &'a BTreeMap<String, Value>,
        _expires: Option<Duration>,
    ) -> impl Future<Output = poem::Result<()>> + Send + 'a {
        async move {
            info!("update session {session_id}");
            info!("{entries:#?}");
            self.db
                .update::<Option<BTreeMap<String, Value>>>(("session", session_id))
                .content(entries)
                .await
                .map(|_| ())
                .map_err(|e| Error::DbException(e).into())
        }
    }

    fn remove_session<'a>(
        &'a self,
        session_id: &'a str,
    ) -> impl Future<Output = poem::Result<()>> + Send + 'a {
        async move {
            info!("remove session {session_id}");
            self.db
                .delete::<Option<BTreeMap<String, Value>>>(("session", session_id))
                .await
                .map(|_| ())
                .map_err(|e| Error::DbException(e).into())
        }
    }
}
