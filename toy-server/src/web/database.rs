use surrealdb::engine::remote::ws::{Client, Ws};
use surrealdb::opt::auth::Root;
use surrealdb::Surreal;

pub(crate) const ROOT_CREDENTIALS: Root = Root {
    username: "root",
    password: "root_pass",
};

pub(crate) async fn connect() -> Result<Surreal<Client>, surrealdb::Error> {
    // let db: Surreal<Client> = Surreal::init();
    // db.connect::<Ws>("ws://localhost:8000").await?;
    let db = Surreal::new::<Ws>("localhost:8000").await?;
    db.use_ns("toy").use_db("toy").await?;

    Ok(db)
}

#[cfg(test)]
mod test {
    use std::collections::BTreeMap;

    use async_std::task::block_on;
    use serde_json::Value;

    use crate::web::database::{connect, ROOT_CREDENTIALS};

    #[test]
    fn db_select() {
        block_on(test_db());
    }

    async fn test_db() {
        println!("1");
        let db = match connect().await {
            Ok(db) => db,
            Err(e) => {
                panic!("连接数据库失败：{e}");
            }
        };
        println!("2");

        if let Err(e) = db.signin(ROOT_CREDENTIALS).await {
            panic!("root用户登录失败: {e}");
        }
        println!("3");

        const SESSION_ID: &str = "9ncx3SiLjSVPq2T2s1niQlg6JiChCRVoG3iEIJ4kCVI";

        let entries = match db
            // .select(("session", "9ncx3SiLjSVPq2T2s1niQlg6JiChCRVoG3iEIJ4kCVI"))
            .query("select * omit id from session where id = $id")
            .bind(("id", SESSION_ID))
            .await
            .and_then(|mut res| res.take(0))
        {
            Ok(Some::<BTreeMap<String, Value>>(mut e)) => {
                // e.remove("id");
                e
            }
            Ok(None) => {
                panic!("select session 为空");
            }
            Err(e) => {
                panic!("select session 失败：{e}");
            }
        };

        println!("load session: {entries:#?}");

        if let Err(e) = db
            .update::<Option<BTreeMap<String, Value>>>(("session", SESSION_ID))
            .content(entries)
            .await
        {
            panic!("update session 失败：{e}");
        }
        println!("success");
    }
}
