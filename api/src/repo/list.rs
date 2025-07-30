use actix_web::{HttpResponse, Responder};
use actix_web::web::{Data, Query};
use serde_json::json;
use infra::App;
use infra::service::repository::RepositoryFilter;
use infra::types::pager::QueryPager;

pub async fn repo_list(
    pager: Query<QueryPager>,
    query: Query<RepositoryFilter>,
    app: Data<App>
) -> impl Responder {
    match app.repository_list(pager.into_inner(), query.into_inner()).await {
        Ok(repos) => HttpResponse::Ok().json(json!({"code": 200, "message": "OK", "data": repos})),
        Err(e) => HttpResponse::Ok().json(json!({"code": 500, "message": e.to_string()})),
    }
}