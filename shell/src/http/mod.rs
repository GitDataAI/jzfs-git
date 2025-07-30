use crate::http::git_receive_pack::git_receive_pack;
use crate::http::git_refs::git_refs;
use crate::http::git_upload_pack::git_upload_pack;
use actix_web::web;
use actix_web::web::{get, post, scope, Data};
use infra::entities::repository::RepositoryModel;
use infra::App;
use std::path::PathBuf;

pub mod git_refs;
pub mod git_receive_pack;
pub mod git_upload_pack;

pub async fn verify_repo_access(
    core: &Data<App>,
    owner: &str,
    repo: &str,
    _require_write: bool
) -> anyhow::Result<PathBuf> {
    let repo_path = RepositoryModel::repository_find_by_owner_name_and_repo_name(&core.db,owner.to_string(), repo.to_string().replace(".git", ""))
        .await
        .map_err(|_| anyhow::anyhow!("Repo not found"))?
        .ok_or(anyhow::anyhow!("Repo not found"))?;
    Ok(repo_path.to_path())
}


pub fn git_route(cfg:&mut web::ServiceConfig) {
    cfg
        .service(
            scope("/git")
                .route("/{owner}/{repo}/git-upload-pack", post().to(git_upload_pack))
                .route("/{owner}/{repo}/git-receive-pack", post().to(git_receive_pack))
                .route("/{owner}/{repo}/info/refs", get().to(git_refs))
        )
    ;
}