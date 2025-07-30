use crate::AppGit;
use crate::tree::msg_tree::GitTreeAuthors;
use git2::*;
use serde::{Deserialize, Serialize};
use std::i32;

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct GitBlobInsertDataParam {
    pub path: String,
    pub file_name: String,
    pub branch: String,
    pub message: String,
    pub content: Vec<u8>,
    pub author: GitTreeAuthors,
    pub committer: GitTreeAuthors,
}

impl AppGit {
    pub fn insert_blob(&self, param: GitBlobInsertDataParam) -> anyhow::Result<String> {
        let repo = self.git()?;
        let oid = repo.blob(&param.content)?;
        let branch_check = Branch::name_is_valid(&format!("refs/heads/{}", param.branch));
        match branch_check {
            Err(e) => {
                return Err(anyhow::anyhow!("Invalid branch name: {}", e));
            }
            Ok(false) => {
                return Err(anyhow::anyhow!("Invalid branch name"));
            }
            _ => {}
        }
        let parent_commit = match repo.find_branch(&param.branch, BranchType::Local) {
            Ok(branch) => {
                let commit = branch.get().peel_to_commit()?;
                Some(commit)
            }
            Err(e) => {
                if e.code() == ErrorCode::NotFound {
                    repo.set_head(&format!("refs/heads/{}", param.branch))?;
                    None
                } else {
                    return Err(anyhow::anyhow!("find_branch error"));
                }
            }
        };
        let tree = parent_commit.clone().map(|x| x.tree());
        let mut tree_builder = match tree {
            Some(Ok(tree)) => repo.treebuilder(Some(&tree))?,
            _ => repo.treebuilder(None)?,
        };
        let file_mode = FileMode::Blob;
        tree_builder.insert(
            match param.path.is_empty() {
                true => param.file_name,
                false => format!("{}/{}", param.path, param.file_name),
            },
            oid,
            i32::from(file_mode),
        )?;
        let new_tree_oid = tree_builder.write()?;
        let new_tree = repo.find_tree(new_tree_oid)?;
        let author = git2::Signature::new(
            &param.author.name,
            &param.author.email,
            &Time::new(param.author.time, 0),
        )?;
        let committer = git2::Signature::new(
            &param.committer.name,
            &param.committer.email,
            &Time::new(param.author.time, 0),
        )?;
        let commit_oid = match parent_commit {
            Some(parent_commit) => {
                let commit_oid = repo.commit(
                    Some(&format!("refs/heads/{}", param.branch)),
                    &author,
                    &committer,
                    &param.message,
                    &new_tree,
                    &[&parent_commit],
                )?;
                commit_oid
            }
            None => {
                let commit_oid = repo.commit(
                    Some(&format!("refs/heads/{}", param.branch)),
                    &author,
                    &committer,
                    &param.message,
                    &new_tree,
                    &[],
                )?;
                commit_oid
            }
        };
        Ok(commit_oid.to_string())
    }
}
