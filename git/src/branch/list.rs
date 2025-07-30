use crate::AppGit;
use git2::BranchType;
use serde::{Deserialize, Serialize};

#[derive(Deserialize,Serialize)]
pub struct GitBranchListResult {
    pub name: String,
    pub head: String,
    pub time: String,
    pub default: bool,
}

impl AppGit {
    pub fn branch_list(&self) -> anyhow::Result<Vec<GitBranchListResult>> {
        let repo = self.git()?;
        let mut branch_list = vec![];
        for branch in repo.branches(Some(BranchType::Local))? {
            let (branch, _) = branch?;
            let branch_name = branch.name()?;
            let branch_head = branch.get().target().map(|x| x.to_string());
            let branch_time = branch.get().peel_to_commit()?.time().seconds().to_string();
            let branch_is_head = branch.is_head();
            branch_list.push(GitBranchListResult {
                name: branch_name.unwrap().to_string(),
                head: branch_head.unwrap_or("".to_string()),
                time: branch_time,
                default: branch_is_head,
            });
        }
        Ok(branch_list)
    }
}
