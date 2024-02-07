use surrealdb::engine::remote::ws::{Client, Ws};
use surrealdb::opt::auth::Root;
use surrealdb::Surreal;

pub(crate) const ROOT_CREDENTIALS: Root = Root {
    username: "root",
    password: "root_pass",
};

pub(crate) async fn connect() -> Result<Surreal<Client>, surrealdb::Error> {
    let db: Surreal<Client> = Surreal::init();
    db.connect::<Ws>("127.0.0.1:8000").await?;

    db.use_ns("toy").use_db("toy").await?;

    Ok(db)
}

#[cfg(test)]
mod test {
    use std::collections::BTreeMap;

    use async_std::task::block_on;
    use serde_json::Value;

    use crate::web::database::connect;

    #[test]
    fn db_select() {
        block_on(test_db());
    }

    async fn test_db() {
        println!("1");
        let db = match connect().await {
            Ok(db) => db,
            Err(e) => {
                println!("连接数据库失败：{e}");
                return;
            }
        };
        println!("2");

        let entries: BTreeMap<String, Value> = match db
            .select(("session", "fd0qOmHiH8xlg_jHjCpokkCzLx0Ggi9UBQi7VB68BmM"))
            .await
        {
            Ok(Some(e)) => e,
            Ok(None) => {
                println!("select session 为空");
                return;
            }
            Err(e) => {
                println!("select session 失败：{e}");
                return;
            }
        };

        println!("load session: {entries:#?}");

        if let Err(e) = db
            .update::<Option<BTreeMap<String, Value>>>((
                "session",
                "fd0qOmHiH8xlg_jHjCpokkCzLx0Ggi9UBQi7VB68BmM",
            ))
            .content(entries)
            .await
        {
            println!("update session 失败：{e}");
        }
        println!("3");
    }
}
