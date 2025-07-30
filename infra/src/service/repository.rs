use crate::entities::repository::RepositoryModel;
use crate::entities::users::UsersModel;
use crate::error::{AppError, AppResult};
use crate::types::pager::QueryPager;
use crate::App;
use chrono::Local;
use git::blob::insert::GitBlobInsertDataParam;
use git::branch::list::GitBranchListResult;
use git::tree::msg_tree::{GitTreeAuthors, StateTreeResult};
use git::tree::state_tree::StateTreeParam;
use git::AppGit;
use serde::{Deserialize, Serialize};
use sqlx::Row;
use uuid::Uuid;

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct RepositoryInitParam {
    pub name: String,
    pub description: String,
    pub initial: bool,
    pub is_public: bool,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct RepositoryFilter {
    order: RepositoryFilterSort,
    name: Option<String>,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub enum RepositoryFilterSort {
    #[serde(rename = "name_asc")]
    NameAsc,
    #[serde(rename = "name_desc")]
    NameDesc,
    #[serde(rename = "created_asc")]
    CreatedAsc,
    #[serde(rename = "created_desc")]
    CreatedDesc,
    #[serde(rename = "updated_asc")]
    UpdatedAsc,
    #[serde(rename = "updated_desc")]
    UpdatedDesc,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct RepositoryModelList {
    pub uid: Uuid,
    pub name: String,
    pub owner: Uuid,
    pub owner_name: String,
    pub description: String,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct RepositoryModelResult {
    pub total: i64,
    pub list: Vec<RepositoryModelList>,
}

impl App {
    pub async fn repository_init(&self, owner: Uuid, param: RepositoryInitParam) -> AppResult<()> {
        let Some(owner) = UsersModel::get_by_uid(&self.db, owner).await? else {
            return Err(AppError::Custom("User not found".to_string()));
        };
        if RepositoryModel::repository_find_by_owner_name_and_repo_name(
            &self.db,
            owner.username.clone(),
            param.name.clone(),
        )
        .await?
        .is_some()
        {
            return Err(AppError::Custom("Repository already exists".to_string()));
        }
        let repo =
            RepositoryModel::create(&self.db, &param.name, owner.uid, &param.description).await?;
        if param.initial {
            let git = AppGit::new(repo.to_path());
            git.init()?;
            let now = Local::now().naive_local();
            git.insert_blob(GitBlobInsertDataParam {
                path: "".to_string(),
                file_name: "README.md".to_string(),
                branch: "main".to_string(),
                message: "Init commit".to_string(),
                content: vec![],
                author: GitTreeAuthors {
                    name: owner.username.to_string(),
                    email: owner.email.to_string(),
                    time: now.and_utc().timestamp(),
                },
                committer: GitTreeAuthors {
                    name: owner.username.to_string(),
                    email: owner.email.to_string(),
                    time: now.and_utc().timestamp(),
                },
            })
            .ok();
        }
        Ok(())
    }
       pub async fn repository_list(
        &self,
        pager: QueryPager,
        filter: RepositoryFilter,
    ) -> AppResult<RepositoryModelResult> {
        let sort_query = match filter.order {
            RepositoryFilterSort::NameAsc => "r.name ASC",
            RepositoryFilterSort::NameDesc => "r.name DESC",
            RepositoryFilterSort::CreatedAsc => "r.created_at ASC",
            RepositoryFilterSort::CreatedDesc => "r.created_at DESC",
            RepositoryFilterSort::UpdatedAsc => "r.updated_at ASC",
            RepositoryFilterSort::UpdatedDesc => "r.updated_at DESC",
        };

        let base_query = "SELECT r.uid, r.owner, u.username as owner_name, r.deleted_at, r.name, r.description, r.updated_at,\
         r.created_at FROM repository r JOIN users u ON u.uid = r.owner".to_string();

        let where_clause = if let Some(ref _name) = filter.name {
            "WHERE r.deleted_at IS NULL AND (r.name LIKE $3 OR r.description LIKE $4)".to_string()
        } else {
            "WHERE r.deleted_at IS NULL".to_string()
        };

        let order_clause = format!("ORDER BY {}", sort_query);
        let limit_clause = "LIMIT $1 OFFSET $2".to_string();

        let query = format!("{} {} {} {}", base_query, where_clause, order_clause, limit_clause);

        let result = if let Some(name) = filter.name.clone() {
            let search_term = format!("%{}%", name);
            sqlx::query(&query)
                .bind(pager.limit)
                .bind(pager.page * pager.limit)
                .bind(&search_term)
                .bind(&search_term)
                .fetch_all(&self.db)
                .await
                .map(|rows| {
                    rows.into_iter()
                        .map(|r| RepositoryModelList {
                            uid: r.get("uid"),
                            name: r.get("name"),
                            owner: r.get("owner"),
                            owner_name: r.get("owner_name"),
                            description: r.get("description"),
                            created_at: r.get("created_at"),
                            updated_at: r.get("updated_at"),
                        })
                        .collect::<Vec<_>>()
                })?
        } else {
            sqlx::query(&query)
                .bind(pager.limit)
                .bind(pager.page * pager.limit)
                .fetch_all(&self.db)
                .await
                .map(|rows| {
                    rows.into_iter()
                        .map(|r| RepositoryModelList {
                            uid: r.get("uid"),
                            name: r.get("name"),
                            owner: r.get("owner"),
                            owner_name: r.get("owner_name"),
                            description: r.get("description"),
                            created_at: r.get("created_at"),
                            updated_at: r.get("updated_at"),
                        })
                        .collect::<Vec<_>>()
                })?
        };

        let count = if let Some(name) = filter.name {
        let search_term = format!("%{}%", name);
        let count_query = format!("SELECT COUNT(*) FROM repository r JOIN users u ON u.uid = r.owner {}", where_clause);
            sqlx::query(&count_query)
                .bind(&search_term)
                .bind(&search_term)
                .fetch_one(&self.db)
                .await
                .map(|x| x.get::<i64, usize>(0))
        } else {
            let count_query = format!("SELECT COUNT(*) FROM repository r JOIN users u ON u.uid = r.owner {}", where_clause);
            sqlx::query(&count_query)
                .fetch_one(&self.db)
                .await
                .map(|x| x.get::<i64, usize>(0))
        }.unwrap_or(0);


        Ok(RepositoryModelResult {
            total: count,
            list: result,
        })
    }

    pub async fn repository_dash(&self, repo: String, owner: String) -> AppResult<RepositoryDashResult> {
        let repo = RepositoryModel::repository_find_by_owner_name_and_repo_name(&self.db, owner, repo).await?
            .ok_or(anyhow::anyhow!("Repository not found"))?;
        let git = AppGit::new(repo.to_path());
        let branches = git.branch_list()?;
        Ok(RepositoryDashResult {
            repo,
            branches,
        })
    }

    pub async fn repository_tree(&self, repo: String, owner: String, path: String) -> AppResult<RepositoryTreeResult> {
        let repo = RepositoryModel::repository_find_by_owner_name_and_repo_name(&self.db, owner, repo).await?
            .ok_or(anyhow::anyhow!("Repository not found"))?;
        let git = AppGit::new(repo.to_path());
        let tree = git.tree_msg(StateTreeParam {
            head: None,
            branch: None,
            path,
        })?;
        Ok(RepositoryTreeResult {
            repo,
            tree,
        })
    }

    pub async fn repository_commits(&self, repo: String, owner: String, page: i32, limit: i32) -> AppResult<RepositoryCommitsResult> {
        let repo = RepositoryModel::repository_find_by_owner_name_and_repo_name(&self.db, owner, repo).await?
            .ok_or(anyhow::anyhow!("Repository not found"))?;
        let git = AppGit::new(repo.to_path());
        let commits_result = git.commit_list(GitCommitListParam {
            start: None,
            end: None,
            limit: Some(limit),
            branch: None
        })?;
        let total = commits_result.total as i32;
        Ok(RepositoryCommitsResult {
            repo,
            commits: commits_result.data,
            total,
            page,
            limit,
        })
    }
    pub async fn repository_branch(&self, repo: String, owner: String) -> AppResult<Vec<GitBranchListResult>> {
        let repo = RepositoryModel::repository_find_by_owner_name_and_repo_name(&self.db, owner, repo).await?
            .ok_or(anyhow::anyhow!("Repository not found"))?;
        let git = AppGit::new(repo.to_path());
        let branch_result = git.branch_list()?;
        Ok(branch_result)
    }
}

#[derive(Deserialize,Serialize)]
pub struct RepositoryDashResult {
    pub repo: RepositoryModel,
    pub branches: Vec<GitBranchListResult>,
}

#[derive(Deserialize,Serialize)]
pub struct RepositoryTreeResult {
    pub repo: RepositoryModel,
    pub tree: StateTreeResult,
}

#[derive(Deserialize,Serialize)]
pub struct RepositoryCommitsResult {
    pub repo: RepositoryModel,
    pub commits: Vec<GitCommit>,
    pub total: i32,
    pub page: i32,
    pub limit: i32,
}

// Assuming GitCommitResult is defined in git module
use git::commit::list::{GitCommit, GitCommitListParam};
