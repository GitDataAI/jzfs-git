use crate::App;
use actix_web::web::{Data, Path};
use actix_web::{HttpResponse, Responder};
use serde_json::json;


pub async fn repo_branch(
    app: Data<App>,
    paths: Path<(String, String)>,
) -> impl Responder {
    let (owner, repo) = paths.into_inner();
    let result = app.repository_branch(repo,owner).await;
    match result {
        Ok(repo) => HttpResponse::Ok().json(json!({"code": 200, "message": "OK", "data": repo})),
        Err(e) => HttpResponse::Ok().json(json!({"code": 500, "message": e.to_string()})),
    }
}
