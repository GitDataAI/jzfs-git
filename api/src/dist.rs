use actix_files::NamedFile;
use actix_web::{HttpRequest, HttpResponse};
use std::path::{Path, PathBuf};
use tokio::fs;

pub async fn dist(req: HttpRequest) -> HttpResponse {
    let base = Path::new("/explore/html");
    let request_path = req.path().trim_matches('/');
    let file_path = resolve_file_path(base, request_path).await;
    match NamedFile::open_async(&file_path).await {
        Ok(named_file) => {
            named_file
                .prefer_utf8(true)
                .use_etag(true)
                .use_last_modified(true)
                .into_response(&req)
        }
        Err(_) => {
            NamedFile::open_async(base.join("index.html"))
                .await
                .map(|f| f.into_response(&req))
                .unwrap_or_else(|_|
                    HttpResponse::NotFound()
                        .content_type("text/html")
                        .body("<h1>404 Not Found</h1>")
                )
        }
    }
}

async fn resolve_file_path(base: &Path, request_path: &str) -> PathBuf {
    if request_path.is_empty() {
        return base.join("index.html");
    }
    let full_path = base.join(request_path);
    if let Ok(metadata) = fs::metadata(&full_path).await {
        if metadata.is_dir() {
            return full_path.join("index.html");
        }
    }
    full_path
}