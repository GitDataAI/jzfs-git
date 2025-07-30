use flate2::bufread::GzDecoder;
use std::io;
use tracing::{span, Level};
use std::io::Cursor;
use bytes::Bytes;
use actix_web::{HttpRequest, HttpResponse, HttpResponseBuilder, Responder};
use actix_web::http::StatusCode;
use actix_web::web::{Data, Path};
use async_stream::stream;
use tracing::{error, info};
use core::AppCore;
use std::path::PathBuf;

#[derive(Debug)]
pub enum TransactionStatus {
    Pending,
    Committed,
    RolledBack,
}

#[derive(Debug)]
pub struct RefChange {
    pub ref_name: String,
    pub old_oid: String,
    pub new_oid: String,
}

#[derive(Debug)]
struct Transaction {
    repo_path: PathBuf,
    changes: Vec<RefChange>,
    status: TransactionStatus,
}

impl Transaction {
    fn new(repo_path: PathBuf) -> Self {
        Self {
            repo_path,
            changes: Vec::new(),
            status: TransactionStatus::Pending,
        }
    }

    fn add_change(&mut self, ref_name: String, old_oid: String, new_oid: String) {
        self.changes.push(RefChange { ref_name, old_oid, new_oid });
    }

    fn commit(&mut self) {
        self.status = TransactionStatus::Committed;
        info!("Transaction committed with changes: {:?}", self.changes);
    }

    fn rollback(&mut self) {
        self.status = TransactionStatus::RolledBack;
        info!("Transaction rolled back");
    }
}
use git::AppGit;
use tokio::process::Command;
use tokio::io::{AsyncWriteExt, AsyncReadExt, AsyncBufReadExt};
use tokio::task;

pub async fn pack(
    request: HttpRequest,
    payload: Bytes,
    path: Path<(String, String)>,
    status: Data<AppCore>,
) -> impl Responder {
    let (owner, repo) = path.into_inner();
    let bytes = if let Some(zip) = request.headers().get("content-encoding") {
        if zip == "gzip" {
            // Use spawn_blocking to handle blocking gzip decoding
            match task::spawn_blocking(move || {
                let mut decoder = GzDecoder::new(Cursor::new(payload));
                let mut decoded_data = Vec::new();
                io::copy(&mut decoder, &mut decoded_data).map(|_| decoded_data)
            }).await {
                Ok(result) => match result {
                    Ok(data) => data,
                    Err(e) => {
                        error!("Gzip decoding failed: {}", e);
                        return HttpResponseBuilder::new(StatusCode::BAD_REQUEST)
                            .body(format!("Gzip decoding error: {}", e));
                    }
                },
                Err(e) => {
                    error!("Decoding task failed: {}", e);
                    return HttpResponseBuilder::new(StatusCode::INTERNAL_SERVER_ERROR)
                        .body("Decoding process failed");
                }
            }
        } else {
            payload.to_vec()
        }
    } else {
        payload.to_vec()
    };

    let version = request.headers().get("Git-Protocol").and_then(|x| x.to_str().ok());

    let mut response = HttpResponse::Ok();
    response
        .insert_header(("Pragma", "no-cache"))
        .insert_header(("Cache-Control", "no-cache, max-age=0, must-revalidate"))
        .insert_header(("Expires", "Fri, 01 Jan 1980 00:00:00 GMT"))
        .insert_header(("Strict-Transport-Security", "max-age=31536000; includeSubDomains; preload"))
        .insert_header(("X-Frame-Options", "DENY"));

    let url = request.uri().path().split("/")
        .filter(|x| !x.is_empty())
        .collect::<Vec<_>>();

    let git_cmd = if url.iter().any(|x| x.contains("git-upload-pack")) {
        response.insert_header(("Content-Type", "application/x-git-upload-pack-result"));
        "upload-pack"
    } else if url.iter().any(|x| x.contains("git-receive-pack")) {
        response.insert_header(("Content-Type", "application/x-git-receive-pack-result"));
        "receive-pack"
    } else {
        return HttpResponseBuilder::new(StatusCode::BAD_REQUEST)
            .body("Unsupported Git protocol");
    };

    let repo_name = repo.replace(".git", "");
    let repo = match status.repo_owner_find(owner.clone(), repo_name.clone()).await {
        Ok(repo) => repo.0,
        Err(_) => {
            error!("Repository not found: {} / {}", owner, repo_name);
            return HttpResponseBuilder::new(StatusCode::NOT_FOUND)
                .body("Repository not found");
        }
    };

    let repo_path = AppGit::from(repo).path_buf;
    let mut transaction = Transaction::new(repo_path.clone());
    if !repo_path.exists() {
        error!("Repository path does not exist: {:?}", repo_path);
        return HttpResponseBuilder::new(StatusCode::NOT_FOUND)
            .body("Repository path does not exist");
    }

    let mut cmd = Command::new("git");
    cmd.arg(git_cmd)
       .arg("--stateless-rpc")
       .arg(".")
       .current_dir(repo_path)
       .stdin(std::process::Stdio::piped())
       .stdout(std::process::Stdio::piped())
       .stderr(std::process::Stdio::piped());

    if let Some(version) = version {
        cmd.env("GIT_PROTOCOL", version);
    }

    let mut child = match cmd.spawn() {
        Ok(child) => child,
        Err(e) => {
            error!("Failed to start Git command: {}", e);
            return HttpResponseBuilder::new(StatusCode::INTERNAL_SERVER_ERROR)
                .body(format!("Command execution failed: {}", e));
        }
    };

    let mut stdin = match child.stdin.take() {
        Some(stdin) => stdin,
        None => {
            error!("Failed to get child process standard input");
            return HttpResponseBuilder::new(StatusCode::INTERNAL_SERVER_ERROR)
                .body("Failed to get standard input");
        }
    };

    if let Err(e) = stdin.write_all(&bytes).await {
        error!("Failed to write data to Git command: {}", e);
        return HttpResponseBuilder::new(StatusCode::BAD_REQUEST)
            .body(format!("Data transfer failed: {}", e));
    }
    drop(stdin);

    let payload_str = String::from_utf8_lossy(&bytes);
    for line in payload_str.lines() {
        let parts: Vec<&str> = line.splitn(3, ' ').collect();
        if parts.len() == 3 && parts[0] != "0000" {
            let old_oid = parts[0].to_string();
            let new_oid = parts[1].to_string();
            let ref_name = parts[2].to_string();
            transaction.add_change(ref_name, old_oid, new_oid);
        }
    }

    let stdout = match child.stdout.take() {
        Some(stdout) => stdout,
        None => {
            error!("Failed to get child process standard output");
            return HttpResponseBuilder::new(StatusCode::INTERNAL_SERVER_ERROR)
                .body("Failed to get standard output");
        }
    };

    if let Some(stderr) = child.stderr.take() {
        tokio::spawn(async move {
            let mut reader = tokio::io::BufReader::new(stderr);
            let mut buffer = String::new();
            while let Ok(n) = reader.read_line(&mut buffer).await {
                if n == 0 {
                    break;
                }
                error!("Git command error output: {}", buffer.trim_end());
                buffer.clear();
            }
        });
    }

    let body = actix_web::body::BodyStream::new(stream! {
        let mut reader = tokio::io::BufReader::new(stdout);
        let mut buffer = [0; 8192];
        loop {
            match reader.read(&mut buffer).await {
                Ok(0) => break,
                Ok(n) => yield Ok::<Bytes, actix_web::Error>(Bytes::copy_from_slice(&buffer[..n])),
                Err(e) => {
                    error!("Failed to read Git command output: {}", e);
                    break;
                }
            }
        }
        
        match child.wait().await {
            Ok(status) if status.success() => {
                info!("Git command executed successfully");
                transaction.commit();
            }
            Ok(status) => {
                error!("Git command execution failed: exit code {:?}", status.code());
                transaction.rollback();
            }
            Err(e) => {
                error!("Failed to wait for Git command: {}", e);
                transaction.rollback();
            }
        }
    });

    response.body(body)
}


pub async fn git_receive_pack(
    request: HttpRequest,
    payload: Bytes,
    path: Path<(String, String)>,
    status: Data<AppCore>,
) -> impl Responder {
    let (owner, repo) = path.into_inner();
    todo!()
}
pub async fn git_upload_pack(
    request: HttpRequest,
    payload: Bytes,
    path: Path<(String, String)>,
    status: Data<AppCore>,
) -> impl Responder {
    let (owner, repo) = path.into_inner();
    todo!()
}