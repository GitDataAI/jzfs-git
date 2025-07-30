use actix_web::web::{Data, Json};
use actix_web::{HttpResponse, Responder};
use infra::service::auth::AuthSignUpParam;
use infra::types::session::AuthSessionExt;
use infra::App;
use rsession::Session;
use serde_json::json;

pub async fn auth_register(
    session: Session,
    param: Json<AuthSignUpParam>,
    app: Data<App>,
) -> impl Responder {
    match app.service_auth_signup(param.0).await {
        Ok(res) => {
            session.set_auth_session(res.clone()).await;
            HttpResponse::Ok()
                .json(json!({"data": res, "message": "Register success", "code": 200}))
        }
        Err(err) => HttpResponse::Ok().json(json!({"message": err.to_string(), "code": 400 })),
    }
}
