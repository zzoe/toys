use poem::session::Session;
use poem::web::Json;
use poem::{handler, Endpoint, Middleware, Request};
use surrealdb::opt::auth::{Jwt, Scope};

use toy_schema::sign::SignReq;

use crate::error::Error::{SignInFail, SignUpFail, UnAuthorized};
use crate::error::ErrorConv;
use crate::web::database;

#[handler]
pub async fn sign_up(sign_req: Json<SignReq>, session: &Session) -> poem::Result<String> {
    let db = database::connect().await.internal_server_error()?;
    let credentials = Scope {
        namespace: "toy",
        database: "toy",
        scope: "user_scope",
        params: sign_req.0,
    };

    let token: Jwt = db.signup(credentials).await.bad_request(SignUpFail)?;

    session.set("token", token);

    Ok("注册成功".to_string())
}

#[handler]
pub async fn sign_in(sign_req: Json<SignReq>, session: &Session) -> poem::Result<String> {
    let db = database::connect().await.internal_server_error()?;
    let credentials = Scope {
        namespace: "toy",
        database: "toy",
        scope: "user_scope",
        params: sign_req.0,
    };

    let token: Jwt = db.signin(credentials).await.bad_request(SignInFail)?;

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
            return Err(poem::error::Unauthorized(UnAuthorized));
        };

        // 从session取token
        let Some(token) = session.get::<Jwt>("token") else {
            return Err(poem::error::Unauthorized(UnAuthorized));
        };

        // 创建数据连接
        let db = database::connect().await.internal_server_error()?;

        // 数据库用户认证
        db.authenticate(token).await.unauthorized()?;

        // 保存数据库连接到req
        req.extensions_mut().insert(db);

        self.ep.call(req).await
    }
}
