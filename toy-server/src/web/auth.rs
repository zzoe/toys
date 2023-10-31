use poem::http::StatusCode;
use poem::session::Session;
use poem::web::Json;
use poem::{handler, Endpoint, Middleware, Request};
use serde::{Deserialize, Serialize};
use surrealdb::opt::auth::{Jwt, Scope};

use crate::error::Error::SessionNotFound;
use crate::error::{Error, SIGN_IN_ERROR, SIGN_UP_ERROR};
use crate::web::database;

#[derive(Deserialize, Serialize)]
pub struct SignReq {
    pub name: String,
    pub email: String,
    pub password: String,
}

#[handler]
pub async fn sign_up(sign_req: Json<SignReq>, session: &Session) -> poem::Result<String> {
    let db = database::new().await?;
    let credentials = Scope {
        namespace: "toy",
        database: "toy",
        scope: "user",
        params: sign_req.0,
    };

    let token: Jwt = db
        .signup(credentials)
        .await
        .map_err(|e| Error::from(e).to_poem_error(StatusCode::BAD_REQUEST, SIGN_UP_ERROR))?;

    session.set("token", token);

    Ok("注册成功".to_string())
}

#[handler]
pub async fn sign_in(sign_req: Json<SignReq>, session: &Session) -> poem::Result<String> {
    let db = database::new().await?;
    let credentials = Scope {
        namespace: "toy",
        database: "toy",
        scope: "user",
        params: sign_req.0,
    };

    let token: Jwt = db
        .signin(credentials)
        .await
        .map_err(|e| Error::from(e).to_poem_error(StatusCode::BAD_REQUEST, SIGN_IN_ERROR))?;

    session.set("token", token);

    Ok("登录成功".to_string())
}

pub struct Auth {}

impl<E: Endpoint> Middleware<E> for Auth {
    type Output = AuthEndpoint<E>;

    fn transform(&self, ep: E) -> Self::Output {
        AuthEndpoint { ep }
    }
}

pub struct AuthEndpoint<E> {
    ep: E,
}

#[poem::async_trait]
impl<E: Endpoint> Endpoint for AuthEndpoint<E> {
    type Output = E::Output;

    async fn call(&self, mut req: Request) -> poem::Result<Self::Output> {
        // 从req取session
        let Some(session) = req.extensions().get::<Session>() else {
            return Err(SessionNotFound.into());
        };

        // 从session取token
        let Some(token) = session.get::<Jwt>("token") else {
            return Err(poem::Error::from_status(StatusCode::UNAUTHORIZED));
        };

        // 创建数据连接
        let db = database::new().await?;
        // 数据库用户认证
        db.authenticate(token).await.map_err(Error::from)?;

        // 保存数据库连接到req
        req.extensions_mut().insert(db);

        self.ep.call(req).await
    }
}
