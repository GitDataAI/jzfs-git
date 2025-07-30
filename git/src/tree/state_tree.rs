use crate::AppGit;
use git2::TreeWalkResult;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct StateTreeParam {
    pub head: Option<String>,
    pub branch: Option<String>,
    pub path: String,
}

#[derive(Deserialize, Serialize, Clone, Debug, Eq, PartialEq, Hash)]
pub struct GitTreeStateFileMapMiddleData {
    pub path: String,
    pub name: String,
    pub rtype: String,
}

impl GitTreeStateFileMapMiddleData {
    pub fn to_path(&self) -> String {
        format!("{}{}", self.path, self.name)
    }
}

impl AppGit {
    pub fn state_tree(
        &self,
        param: StateTreeParam,
    ) -> anyhow::Result<Vec<GitTreeStateFileMapMiddleData>> {
        let repo = self.git()?;
        let refs = match param.branch {
            Some(branch) => repo
                .find_branch(&branch, git2::BranchType::Local)?
                .into_reference(),
            None => repo.head()?,
        };
        let commit = match param.head {
            Some(oid) => repo.find_commit(git2::Oid::from_str(&oid)?)?,
            None => repo.find_commit(
                refs.target()
                    .ok_or(anyhow::anyhow!("Failed to get target from reference"))?,
            )?,
        };
        let mut param_path = param
            .path
            .split("/")
            .map(|s| s.trim())
            .filter(|s| !s.is_empty())
            .collect::<Vec<_>>()
            .join("/");
        if !param_path.is_empty() && !param_path.ends_with("/") {
            param_path.push_str("/");
        }
        let tree = commit.tree()?;
        let mut result = vec![];
        tree.walk(git2::TreeWalkMode::PreOrder, |root, entry| {
            let name = entry.name().unwrap_or("?").to_string();
            let path = format!("{}{}", root, entry.name().unwrap_or("?"));
            if root == param_path {
                match entry.kind() {
                    Some(git2::ObjectType::Tree) => {
                        result.push(GitTreeStateFileMapMiddleData {
                            name: name.clone(),
                            path: path.clone().replace(&name, ""),
                            rtype: "tree".to_string(),
                        });
                    }
                    Some(git2::ObjectType::Blob) => result.push(GitTreeStateFileMapMiddleData {
                        name: name.clone(),
                        path: path.clone().replace(&name, ""),
                        rtype: "blob".to_string(),
                    }),
                    _ => {}
                }
            }
            TreeWalkResult::Ok
        })
        .ok();
        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;
    #[test]
    fn test_git_state_tree() {
        let git = AppGit::new(PathBuf::from("E:\\Code\\acl-anthology.git"));
        let commits = git.state_tree(StateTreeParam {
            head: Some("cff06e89b75f3b73059430bf2162bf8341e96222".to_string()),
            branch: None,
            path: "bin/fixedcase".to_string(),
        });
        // dbg!(commits);
    }
}
