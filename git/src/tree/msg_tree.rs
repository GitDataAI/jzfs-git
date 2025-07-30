use crate::AppGit;
use crate::tree::state_tree::{GitTreeStateFileMapMiddleData, StateTreeParam};
use git2::DiffOptions;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::io;

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct StateTreeResult {
    pub data: Vec<GitTreeStateFileMap>,
    pub file: Vec<(GitTreeStateFileMapMiddleData, usize)>,
    pub authors: HashMap<String, GitTreeAuthors>,
}

impl StateTreeResult {
    pub fn insert_authors(&mut self, authors: GitTreeAuthors) -> String {
        if let Some(index) = self
            .authors
            .iter()
            .find(|(_, v)| {
                return if v.name == authors.name && v.email == authors.email {
                    true
                } else {
                    false
                };
            })
            .map(|x| x.0.clone())
        {
            index
        } else {
            self.authors
                .insert((self.authors.len() + 1).to_string(), authors);
            (self.authors.len() + 1).to_string()
        }
    }
    pub fn insert_data(&mut self, data: GitTreeStateFileMap) -> usize {
        if let Some(index) = self.data.iter().find(|x| x == &&data) {
            index.index
        } else {
            self.data.push(data.clone());
            data.index
        }
    }
}

#[derive(Deserialize, Serialize, Clone, Debug, Eq, PartialEq, Hash)]
pub struct GitTreeStateFileMap {
    pub index: usize,
    pub author: String,
    pub committer: String,
    pub message: String,
    pub timestamp: i64,
    pub oid: String,
}

#[derive(Deserialize, Serialize, Clone, Debug, Eq, PartialEq, Hash)]
pub struct GitTreeAuthors {
    pub name: String,
    pub email: String,
    pub time: i64,
}

impl AppGit {
    pub fn tree_msg(&self, param: StateTreeParam) -> anyhow::Result<StateTreeResult> {
        let state_tree = self.state_tree(param.clone())?;
        let repo = self.git()?;
        let refs = match param.branch {
            Some(branch) => repo
                .find_branch(&branch, git2::BranchType::Local)?
                .into_reference(),
            None => repo.head()?,
        };
        let mut commit = match param.head {
            Some(oid) => repo.find_commit(git2::Oid::from_str(&oid)?)?,
            None => repo.find_commit(
                refs.target()
                    .ok_or(anyhow::anyhow!("Failed to get target from reference"))?,
            )?,
        };
        let mut result = StateTreeResult {
            data: vec![],
            file: Default::default(),
            authors: Default::default(),
        };
        let mut has = vec![];
        loop {
            let parent = match commit.parent(0) {
                Ok(parent) => parent,
                Err(_) => break,
            };
            let now_tree = commit
                .tree()
                .map_err(|e| io::Error::new(io::ErrorKind::Other, e.message()))?;
            let pre_tree = parent
                .tree()
                .map_err(|e| io::Error::new(io::ErrorKind::Other, e.message()))?;
            let diff = repo
                .diff_tree_to_tree(
                    Some(&pre_tree),
                    Some(&now_tree),
                    Some(&mut DiffOptions::new()),
                )
                .map_err(|e| io::Error::new(io::ErrorKind::Other, e.message()))?;
            let author = GitTreeAuthors {
                name: commit.author().name().unwrap_or("?").to_string(),
                email: commit.author().email().unwrap_or("?").to_string(),
                time: commit.time().seconds(),
            };
            let committer = GitTreeAuthors {
                name: commit.committer().name().unwrap_or("?").to_string(),
                email: commit.committer().email().unwrap_or("?").to_string(),
                time: commit.time().seconds(),
            };
            let author_index = result.insert_authors(author);
            let committer_index = result.insert_authors(committer);
            let msg = GitTreeStateFileMap {
                index: result.data.len(),
                author: author_index,
                committer: committer_index,
                message: commit.message().unwrap_or("?").to_string(),
                timestamp: commit.time().seconds(),
                oid: commit.id().to_string(),
            };
            for delta in diff.deltas() {
                let file = delta.new_file();
                if let Some(path) = file.path() {
                    if let Some(Some(parent_dir)) = path.parent().map(|x|x.to_str().map(|x|x.to_string())) {
                        if !has.contains(&parent_dir.to_string()) {
                            if let Some(item) = state_tree.iter().find(|x| x.to_path().starts_with(&parent_dir)) {
                                if item.rtype == "tree".to_string() {
                                    let msg_index = result.insert_data(msg.clone());
                                    result.file.push((item.clone(), msg_index));
                                    has.push(parent_dir.to_string());
                                }
                            }
                        }
                    }
                    if let Some(path) = path.to_str().map(|x| x.to_string()) {
                        if has.contains(&path) {
                            continue;
                        }
                        if let Some(item) = state_tree.iter().find(|x| x.to_path() == path) {
                            if item.rtype == "blob".to_string() {
                                let msg_index = result.insert_data(msg.clone());
                                result.file.push((item.clone(), msg_index));
                                has.push(item.to_path());
                            }
                        }
                    }
                }
            }
            if has.len() == state_tree.len() {
                break;
            }
            commit = parent;
        }
        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;
    #[test]
    fn test_git_msg_tree() {
        let git = AppGit::new(PathBuf::from("E:\\Code\\acl-anthology.git"));
        let commits = git.tree_msg(StateTreeParam {
            head: None,
            branch: None,
            path: "".to_string(),
        });
        dbg!(commits.unwrap());
    }
}
