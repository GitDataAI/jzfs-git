use actix_web::{HttpResponse, Responder};
use actix_web::web::{Data, Path};
use serde_json::json;
use infra::App;

pub async fn repo_tree(
    path: Path<(String, String, String)>,
    app: Data<App>
) -> impl Responder {
    let (owner, repo, file_path) = path.into_inner();
    match app.repository_tree(repo, owner, file_path).await {
        Ok(tree) => HttpResponse::Ok().json(json!({"code": 200, "message": "OK", "data": tree})),
        Err(e) => HttpResponse::Ok().json(json!({"code": 500, "message": e.to_string()})),
    }
}