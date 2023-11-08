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
