use poem::session::Session;
use poem::{handler, Endpoint, Middleware, Request};
use surrealdb::opt::auth::{Jwt, Scope};
use tracing::{debug, error, info, warn};

use toy_schema::sign::SignReq;

use crate::error::Error;
use crate::web::database;
use crate::web::speedy_data::Speedy;

#[handler]
pub async fn sign_up(sign_req: Speedy<SignReq>, session: &Session) -> poem::Result<Speedy<()>> {
    debug!("sign_up session: {session:#?}");
    if session.get::<Jwt>("token").is_some() {
        session.renew();
    }

    let db = database::connect().await.map_err(Error::DbException)?;
    let credentials = Scope {
        namespace: "toy",
        database: "toy",
        scope: "user_scope",
        params: sign_req.0,
    };

    let token: Jwt = db
        .signup(credentials)
        .await
        .map_err(|_| Error::SignUpFail)?;

    session.set("token", token);

    Ok(Speedy(()))
}

#[handler]
pub async fn sign_in(sign_req: Speedy<SignReq>, session: &Session) -> poem::Result<Speedy<()>> {
    debug!("sign_in session: {session:#?}");
    if session.get::<Jwt>("token").is_some() {
        session.renew();
    }

    let db = database::connect().await.map_err(Error::DbException)?;
    let credentials = Scope {
        namespace: "toy",
        database: "toy",
        scope: "user_scope",
        params: sign_req.0,
    };

    let token: Jwt = db
        .signin(credentials)
        .await
        .map_err(|_| Error::SignInFail)?;

    session.set("token", token);

    Ok(Speedy(()))
}

#[handler]
pub async fn sign_check(session: &Session) -> poem::Result<Speedy<bool>> {
    debug!("sign_check session: {session:#?}");
    let Some(token) = session.get::<Jwt>("token") else {
        warn!("session已失效，未获取到数据库token");
        session.purge();
        return Ok(Speedy(false));
    };

    let db = match database::connect().await {
        Ok(d) => d,
        Err(e) => {
            warn!("数据库连接失败：{e}");
            session.purge();
            return Err(Error::InternalServerErr.into());
        }
    };

    if let Err(e) = db.authenticate(token).await {
        warn!("数据库token验证失败：{e}");
        session.purge();
        return Ok(Speedy(false));
    }

    Ok(Speedy(true))
}

#[handler]
pub async fn logout(session: &Session) -> poem::Result<Speedy<()>> {
    session.purge();
    Ok(Speedy(()))
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
            error!("未读取到session");
            return Err(Error::UnAuthenticated.into());
        };

        // 从session取token
        let Some(token) = session.get::<Jwt>("token") else {
            error!("session中未读取到token");
            return Err(Error::UnAuthenticated.into());
        };

        // 创建数据连接
        let db = database::connect().await.map_err(Error::DbException)?;
        info!("数据库连接成功");

        // 数据库用户认证
        db.authenticate(token).await.map_err(Error::DbException)?;
        info!("数据库用户认证成功");

        // 保存数据库连接到req
        req.extensions_mut().insert(db);

        self.ep.call(req).await
    }
}
