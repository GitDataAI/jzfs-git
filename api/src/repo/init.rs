use actix_web::HttpResponse;
use actix_web::web::{Data, Json};
use rsession::Session;
use serde_json::json;
use infra::App;
use infra::service::repository::RepositoryInitParam;
use infra::types::session::AuthSessionExt;

pub async fn repo_init(
    param: Json<RepositoryInitParam>,
    app: Data<App>,
    session: Session,
) -> impl actix_web::Responder {
    let Some(user) = session.to_auth().await else {
        return HttpResponse::Ok()
            .json(json!({"code": 401, "message": "Not login"}))
    };
    match app.repository_init(user.uid, param.into_inner()).await {
        Ok(repo) => HttpResponse::Ok().json(json!({"code": 200, "message": "OK", "data": repo})),
        Err(e) => HttpResponse::Ok().json(json!({"code": 500, "message": e.to_string()})),
    }
}
