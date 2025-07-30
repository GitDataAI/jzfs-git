use crate::AppGit;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct GitTreeParam {
    pub oid: Option<String>,
    pub branch: Option<String>,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct GitTreeFile {
    pub path: String,
}
#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct GitTreeFileMap {
    pub status: String,
    pub old_file: Option<GitTreeFile>,
    pub new_file: Option<GitTreeFile>,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct GitTreeResult {
    pub data: Vec<GitTreeFileMap>,
}

impl AppGit {
    pub fn commit_tree(&self, param: GitTreeParam) -> anyhow::Result<()> {
        let repo = self.git()?;
        let refs = match param.branch {
            Some(branch) => repo
                .find_branch(&branch, git2::BranchType::Local)?
                .into_reference(),
            None => repo.head()?,
        };
        let commit = match param.oid {
            Some(oid) => repo.find_commit(git2::Oid::from_str(&oid)?)?,
            None => repo.find_commit(
                refs.target()
                    .ok_or(anyhow::anyhow!("Failed to get target from reference"))?,
            )?,
        };
        let parent = commit.parent(0);

        let tree = match parent {
            Ok(parent) => {
                repo.diff_tree_to_tree(Some(&parent.tree()?), Some(&commit.tree()?), None)
            }
            Err(_) => repo.diff_tree_to_tree(None, Some(&commit.tree()?), None),
        };
        let diff = match tree {
            Ok(tree) => tree,
            Err(e) => return Err(anyhow::anyhow!("Failed to get tree: {}", e)),
        };
        let mut tree_map = vec![];
        for delta in diff.deltas() {
            let old_file = delta.old_file();
            let new_file = delta.new_file();
            let old_file_path = match old_file.path() {
                Some(path) => path.to_str().unwrap().to_string(),
                None => "".to_string(),
            };
            let new_file_path = match new_file.path() {
                Some(path) => path.to_str().unwrap().to_string(),
                None => "".to_string(),
            };
            let status = match delta.status() {
                git2::Delta::Added => "added",
                git2::Delta::Deleted => "deleted",
                git2::Delta::Modified => "modified",
                git2::Delta::Renamed => "renamed",
                git2::Delta::Copied => "copied",
                git2::Delta::Ignored => "ignored",
                git2::Delta::Untracked => "untracked",
                git2::Delta::Unmodified => "unmodified",
                git2::Delta::Typechange => "typechange",
                git2::Delta::Unreadable => "unreadable",
                git2::Delta::Conflicted => "conflicted",
            }
            .to_string();
            let map = GitTreeFileMap {
                status,
                old_file: Some(GitTreeFile {
                    path: old_file_path,
                }),
                new_file: Some(GitTreeFile {
                    path: new_file_path,
                }),
            };
            tree_map.push(map);
        }
        dbg!(tree_map);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;
    #[test]
    fn git_commit_tree() {
        let git = AppGit::new(PathBuf::from("E:\\Code\\acl-anthology.git"));
        let commits = git.commit_tree(GitTreeParam {
            oid: Some("1343dbe7cfdcf54aba90956fc2ad4175a4d98042".to_string()),
            branch: None,
        });
        dbg!(commits.ok());
    }
}
