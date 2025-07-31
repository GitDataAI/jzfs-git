use std::collections::HashMap;
use actix_web::{HttpResponse, Responder};
use actix_web::web::{Data, Path, Query};
use serde_json::json;
use infra::App;

pub async fn repo_cat_file(
    path: Path<(String,String,String)>,
    app: Data<App>,
    query: Query<HashMap<String,String>>
) -> impl Responder {
    let (owner,repo,path) = path.into_inner();
    let repo = match app.repository_dash(repo,owner).await {
        Ok(x) => x.repo,
        Err(e) => return HttpResponse::Ok()
            .json(json!({ "code": 500, "message": e.to_string()})),
    };
    let branch = query.get("branch").map(|x|x.clone());
    let commit = query.get("commit").map(|x|x.clone());
    match app.cat_file(repo,path.as_ref(),branch,commit).await {
        Ok(x) => HttpResponse::Ok()
            .body(x),
        Err(e) => HttpResponse::Ok()
            .json(json!({ "code": 500, "message": e.to_string()})),
    }
}