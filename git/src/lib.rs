use anyhow::Context;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

pub fn root_data() -> PathBuf {
    dotenv::dotenv().ok();
    let path_buf = PathBuf::from(std::env::var("ROOT_DATA").expect("ROOT_DATA not set"));
    if !path_buf.exists() {
        std::fs::create_dir_all(&path_buf).expect("Failed to create root data directory");
    }
    path_buf
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct AppGit {
    pub path_buf: PathBuf,
}

impl AppGit {
    pub fn new(path_buf: PathBuf) -> Self {
        AppGit {
            path_buf: root_data().join(path_buf),
        }
    }
    pub fn git(&self) -> anyhow::Result<git2::Repository> {
        git2::Repository::open_bare(self.path_buf.as_path())
            .with_context(|| "Invalid git repository")
    }
    pub fn init(&self) -> anyhow::Result<()> {
        if let Some(parent) = self.path_buf.parent() {
            if !parent.exists() {
                std::fs::create_dir_all(parent)?;
            }
        }
        git2::Repository::init_bare(self.path_buf.as_path())
            .with_context(|| "Invalid git repository")?;
        Ok(())
    }
    pub fn exists(&self) -> bool {
        self.path_buf.as_path().exists() && self.git().is_ok()
    }
}

pub mod blob;
pub mod branch;
pub mod commit;
pub mod remote;
pub mod tag;
pub mod tree;
pub mod cat_file;