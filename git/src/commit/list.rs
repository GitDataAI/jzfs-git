use crate::AppGit;
use git2::Oid;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct GitCommitListParam {
    pub start: Option<String>,
    pub end: Option<String>,
    pub limit: Option<i32>,
    pub branch: Option<String>,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct GitCommitListResult {
    pub data: Vec<GitCommit>,
    pub total: usize,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct GitCommit {
    pub hash: String,
    pub author: String,
    pub email: String,
    pub committer: String,
    pub committer_email: String,
    pub message: String,
    pub timestamp: i64,
}

impl AppGit {
    pub fn commit_list(&self, param: GitCommitListParam) -> anyhow::Result<GitCommitListResult> {
        let repo = self.git()?;
        let refs = match param.branch {
            Some(branch) => repo
                .find_branch(&branch, git2::BranchType::Local)?
                .into_reference(),
            None => repo.head()?,
        };
        let branch_oid = refs
            .target()
            .ok_or(anyhow::anyhow!("Failed to get target from reference"))?;
        let mut revwalk = repo.revwalk()?;
        revwalk.push(branch_oid)?;
        revwalk.set_sorting(git2::Sort::TIME)?;
        let revwalk = revwalk.collect::<Vec<_>>();
        let total = revwalk.len();
        let mut commits = Vec::new();
        let limit = param.limit.unwrap_or(i32::MAX);
        let mut count = 0;
        let start_oid = param.start.map(|x| Oid::from_str(&x));
        let end_oid = param.end.map(|x| Oid::from_str(&x));
        let mut start = false;
        let mut end = false;
        for oid in revwalk {
            if end {
                break;
            }
            let oid = oid?;
            if let Some(Ok(start)) = start_oid {
                if oid == start {
                    end = true;
                }
            }
            if let Some(Ok(end)) = end_oid {
                if oid != end {
                    if !start {
                        continue;
                    }
                } else {
                    start = true;
                }
            }
            let commit = repo.find_commit(oid)?;
            commits.push(GitCommit {
                hash: commit.id().to_string(),
                author: commit
                    .author()
                    .name()
                    .map(|s| s.to_string())
                    .unwrap_or_default(),
                email: commit
                    .author()
                    .email()
                    .map(|s| s.to_string())
                    .unwrap_or_default(),
                committer: commit
                    .committer()
                    .name()
                    .map(|s| s.to_string())
                    .unwrap_or_default(),
                committer_email: commit
                    .committer()
                    .email()
                    .map(|s| s.to_string())
                    .unwrap_or_default(),
                message: commit.message().map(|s| s.to_string()).unwrap_or_default(),
                timestamp: commit.time().seconds(),
            });

            count += 1;
            if count >= limit {
                break;
            }
        }

        Ok(GitCommitListResult {
            data: commits,
            total,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;
    #[test]
    fn test_git_commit_list() {
        let git = AppGit::new(PathBuf::from(
            "E:\\Never\\data\\00000000-0000-0000-0000-000000000000\\019827ba-ae14-7560-8cf2-44d4e12781c8",
        ));
        let commits = git.commit_list(GitCommitListParam {
            start: None,
            end: None,
            limit: Some(100),
            branch: None,
        });
        dbg!(commits.ok());
    }
}
