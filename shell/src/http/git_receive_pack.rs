use std::io;
use std::io::Read;
use std::process::{Command, Stdio};
use crate::http::{verify_repo_access};
use actix_web::{HttpRequest, HttpResponse, Responder};
use actix_web::web::{Data, Path, Payload};
use async_stream::stream;
use bytes::Bytes;
use futures_util::StreamExt;
use infra::App;

pub async fn git_receive_pack(
    _request: HttpRequest,
    mut payload: Payload,
    path: Path<(String, String)>,
    core: Data<App>,
) -> impl Responder {
    let (owner, repo) = path.into_inner();
    let repo_path = match verify_repo_access(&core, &owner, &repo, true).await {
        Ok(p) => p,
        Err(e) => return HttpResponse::Forbidden().body(e.to_string()),
    };
    let mut child = match Command::new("git")
        .arg("receive-pack")
        .arg("--stateless-rpc")
        .arg(repo_path)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
    {
        Ok(c) => c,
        Err(_) => return HttpResponse::InternalServerError().finish(),
    };
    if let Some(mut stdin) = child.stdin.take() {
        use std::io::Write;
        while let Some(Ok(bytes)) = payload.next().await {
            stdin.write_all(&bytes).ok();
        }
    }
    let mut stdout = child.stdout.unwrap();
    let body = actix_web::body::BodyStream::new(stream! {
        let mut buffer = [0; 8192];
        loop {
            match stdout.read(&mut buffer) {
                Ok(0) => {
                    break;
                }
                Ok(n) => {
                    yield Ok::<_, io::Error>(Bytes::copy_from_slice(&buffer[..n]));
                }
                Err(_e) => {
                    break;
                }
            }
        }
    });
    tokio::spawn(async move {
        core.sync_hook_with_owner_repo(repo.replace(".git", ""), owner).await;
    });
    HttpResponse::Ok()
        .content_type("application/x-git-receive-pack-result")
        .body(body)
}