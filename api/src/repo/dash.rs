use actix_web::{HttpResponse, Responder};
use actix_web::web::{Data, Path};
use serde_json::json;
use infra::App;

pub async fn repo_dash(
    path: Path<(String,String)>,
    app: Data<App>
) -> impl Responder {
    let (owner, repo) = path.into_inner();
    match app.repository_dash(repo,owner).await {
        Ok(repo) => HttpResponse::Ok().json(json!({"code": 200, "message": "OK", "data": repo})),
        Err(e) => HttpResponse::Ok().json(json!({"code": 500, "message": e.to_string()})),
    }
}