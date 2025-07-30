use actix_web::{HttpResponse, Responder};
use actix_web::web::{Data, Path, Query};
use serde_json::json;
use infra::App;
use infra::types::pager::QueryPager;

pub async fn repo_commits(
    path: Path<(String, String)>,
    query: Query<QueryPager>,
    app: Data<App>
) -> impl Responder {
    let (owner, repo) = path.into_inner();
    let pager = query.into_inner();
    match app.repository_commits(repo, owner, pager.page, pager.limit).await {
        Ok(commits) => HttpResponse::Ok().json(json!({"code": 200, "message": "OK", "data": commits})),
        Err(e) => HttpResponse::Ok().json(json!({"code": 500, "message": e.to_string()})),
    }
}