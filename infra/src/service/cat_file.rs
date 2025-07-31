use git::AppGit;
use crate::App;
use crate::entities::repository::RepositoryModel;

impl App {
    pub async fn cat_file(&self, repo: RepositoryModel, path: &str, branch: Option<String>, commit: Option<String>) -> anyhow::Result<Vec<u8>> {
        let git = AppGit::new(repo.to_path());
        return git.cat_file(branch, commit, path);
    } 
}