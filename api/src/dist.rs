use actix_files::NamedFile;
use actix_web::{HttpRequest, Responder};

pub async fn dist(req: HttpRequest) -> impl Responder {
    let path = req.path();
    NamedFile::open_async(format!("/explore/html/{}", path))
        .await
        .unwrap_or(NamedFile::open_async("/explore/html/index.html").await.unwrap())
        .prefer_utf8(true)
        .use_etag(true)
        .use_last_modified(true)
}