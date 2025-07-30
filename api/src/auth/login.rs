use actix_web::web::{Data, Json};
use actix_web::{HttpResponse, Responder};
use infra::service::auth::AuthLoginParam;
use infra::types::session::AuthSessionExt;
use infra::App;
use rsession::Session;
use serde_json::json;

pub async fn auth_login(
    session: Session,
    param: Json<AuthLoginParam>,
    app: Data<App>,
) -> impl Responder {
    match app.service_auth_login(param.0).await {
        Ok(res) => {
            session.set_auth_session(res.clone()).await;
            HttpResponse::Ok().json(json!({"data": res, "message": "Login success", "code": 200}))
        }
        Err(err) => HttpResponse::Ok().json(json!({"message": err.to_string(), "code": 400 })),
    }
}


pub async fn auth_logout(
    session: Session
) -> impl Responder {
    session.clear_auth_session().await;
    HttpResponse::Ok().json(json!({"message": "Logout success", "code": 200}))
}