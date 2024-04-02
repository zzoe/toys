use poem::{Endpoint, handler, Middleware, Request};
use poem::session::Session;
use poem::web::Json;
use surrealdb::opt::auth::{Jwt, Scope};
use tracing::{debug, warn};

use toy_schema::sign::SignReq;

use crate::error::Error::{SignInFail, SignUpFail, UnAuthenticated, UnAuthorized};
use crate::error::ErrorConv;
use crate::web::database;

#[handler]
pub async fn sign_up(sign_req: Json<SignReq>, session: &Session) -> poem::Result<String> {
    debug!("sign_up session: {session:#?}");
    if session.get::<Jwt>("token").is_some() {
        session.renew();
        // return Err(poem::error::BadRequest(DoNotRepeat));
    }

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
    debug!("sign_in session: {session:#?}");
    if session.get::<Jwt>("token").is_some() {
        session.renew();
    }

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

#[handler]
pub async fn sign_check(session: &Session) -> poem::Result<String> {
    debug!("sign_check session: {session:#?}");
    let Some(token) = session.get::<Jwt>("token") else {
        warn!("session已失效，未获取到数据库token");
        session.purge();
        return Err(poem::error::Unauthorized(UnAuthenticated));
    };

    let db = match database::connect().await {
        Ok(d) => d,
        Err(e) => {
            warn!("数据库连接失败：{e}");
            session.purge();
            return Err(poem::error::Unauthorized(UnAuthenticated));
        }
    };

    if let Err(e) = db.authenticate(token).await {
        warn!("数据库token验证失败：{e}");
        session.purge();
        return Err(poem::error::Unauthorized(UnAuthenticated));
    }

    Ok("已登录".to_string())
}

#[handler]
pub async fn logout(session: &Session) -> poem::Result<String> {
    session.purge();
    Ok("已登出".to_string())
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
