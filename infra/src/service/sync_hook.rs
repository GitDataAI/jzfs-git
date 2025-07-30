use crate::entities::git_branch::GitBranchModel;
use crate::entities::git_commit::GitCommitModel;
use crate::entities::repository::RepositoryModel;
use crate::error::AppResult;
use crate::App;
use git::commit::list::GitCommitListParam;
use git::AppGit;

impl App {
    
    pub async fn sync_hook_with_owner_repo(&self, owner: String, repo: String) {
        match RepositoryModel::repository_find_by_owner_name_and_repo_name(&self.db, owner, repo).await {
            Ok(Some(x)) => {
                self.sync_hook(x).await.ok();
            }
            Err(_) => {},
            Ok(None) => {}
        }
    }
    pub async fn sync_hook(&self, repo: RepositoryModel) -> AppResult<()> {
        let branches = GitBranchModel::get_by_repo_uid(&self.db, repo.uid).await?;
        // let tags = GitTags::get_by_repo_uid(&self.db, repo.uid).await?;
        let git = AppGit::new(repo.to_path());
        let branch_list = git.branch_list()?;
        for branch in branch_list {
            if branches.iter().find(|x|x.name == branch.name).is_some() {
                if branches.iter().find(|x|x.name == branch.name).unwrap().head != branch.head {
                    GitBranchModel::update(&self.db, branches.iter().find(|x|x.name == branch.name).unwrap().uid, None, Some(&branch.head)).await?;
                }
            } else {
                GitBranchModel::create(&self.db, repo.uid, &branch.name, &branch.head).await?;
            }
        }
        let branches = GitBranchModel::get_by_repo_uid(&self.db, repo.uid).await?;
        for branch in branches {
            if let Ok(commit_list) = git.commit_list(GitCommitListParam {
                start: None,
                end: None,
                limit: None,
                branch: Some(branch.name.clone()),
            }) {
                for cmt in commit_list.data {
                    if let None = GitCommitModel::get_by_sha(&self.db, &cmt.hash).await? {
                        GitCommitModel::create(
                            &self.db,
                            cmt.hash.as_str(),
                            branch.uid,
                            repo.uid,
                            branch.name.clone().as_str(),
                            cmt.message.as_str(),
                            cmt.author.as_str(),
                            cmt.email.as_str(),
                            cmt.committer.as_str(),
                            cmt.committer_email.as_str(),
                        )
                            .await?;
                    }
                }
            }
        }

        Ok(())
    }
}