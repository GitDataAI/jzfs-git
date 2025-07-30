use crate::AppGit;
use anyhow::anyhow;
use git2::Oid;

pub struct RepoGetBytes {
    pub branch: Option<String>,
    pub path: String,
    pub sha1: Option<String>,
}

impl AppGit {
    pub fn blob_bytes(&self, param: RepoGetBytes) -> anyhow::Result<Vec<u8>> {
        let repo = self.git()?;
        let refs = match param.branch {
            Some(x) => repo
                .find_branch(&x, git2::BranchType::Local)?
                .into_reference(),
            None => repo.head()?,
        };
        let commit = match param.sha1 {
            Some(x) => repo.find_commit(Oid::from_str(&x)?)?,
            None => refs.peel_to_commit()?,
        };
        let tree = commit.tree()?;
        let entry = tree.get_path((&param.path).as_ref())?;
        let object = entry.to_object(&repo)?;
        let blob = object.as_blob().ok_or(anyhow!("object is not blob"))?;
        Ok(blob.content().to_vec())
    }
}
