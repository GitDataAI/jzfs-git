use crate::AppGit;
use std::path::Path;

impl AppGit {
    pub fn cat_file(&self, branch: Option<String>, commit: Option<String>, path: &str) -> anyhow::Result<Vec<u8>> {
        let repo = self.git()?;
        let refs = match branch {
            Some(branch) => repo
                .find_branch(&branch, git2::BranchType::Local)?
                .into_reference(),
            None => repo.head()?,
        };
        let commit = match commit {
            Some(oid) => repo.find_commit(git2::Oid::from_str(&oid)?)?,
            None => repo.find_commit(
                refs.target()
                    .ok_or(anyhow::anyhow!("Failed to get target from reference"))?,
            )?,
        };
        let tree = commit.tree()?;
        let tree = tree.get_path(Path::new(path))?;
        let object = tree.to_object(&repo)?;
        let blob = object.as_blob()
            .ok_or(anyhow::anyhow!("Failed to get blob from object"))?;
        return Ok(blob.content().to_vec());
    }
}