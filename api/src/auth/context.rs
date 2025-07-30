use actix_web::{HttpResponse, Responder};
use infra::types::session::AuthSessionExt;
use rsession::Session;
use serde_json::json;

pub async fn auth_context(session: Session) -> impl Responder {
    match session.to_auth().await {
        None => HttpResponse::Ok().json(json!({"data": {}, "message": "Not login", "code": 401})),
        Some(x) => {
            HttpResponse::Ok().json(json!({"data": x, "message": "Login success", "code": 200}))
        }
    }
}
