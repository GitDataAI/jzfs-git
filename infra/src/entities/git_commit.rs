use chrono::Local;
use serde::{Deserialize, Serialize};
use sqlx::{Decode, Encode, Error, FromRow, PgPool, Row};
use uuid::Uuid;
use crate::error::AppResult;

#[derive(Deserialize, Serialize, Encode, Decode, FromRow)]
pub struct GitCommitModel {
    #[sqlx(primary_key)]
    pub uid: Uuid,
    pub sha: String,
    pub branch_uid: Uuid,
    pub repo_uid: Uuid,
    pub branch_name: String,
    pub message: String,
    pub author_name: String,
    pub author_email: String,
    pub commiter_name: String,
    pub commiter_email: String,
    pub timestamp: i64,
    pub created_at: chrono::NaiveDateTime,
}

impl GitCommitModel {
    pub async fn create(
        pool: &PgPool,
        sha: &str,
        branch_uid: Uuid,
        repo_uid: Uuid,
        branch_name: &str,
        message: &str,
        author_name: &str,
        author_email: &str,
        commiter_name: &str,
        commiter_email: &str,
    ) -> Result<GitCommitModel, Error> {
        let uid = Uuid::new_v4();
        let timestamp = Local::now().timestamp();
        let created_at = Local::now().naive_local();
        let row = sqlx::query(
            r#"
        INSERT INTO git_commit (
            uid, sha, branch_uid, repo_uid, branch_name, message, 
            author_name, author_email, commiter_name, commiter_email, 
            timestamp, created_at
        )
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12)
        RETURNING *
        "#,
        )
        .bind(uid)
        .bind(sha)
        .bind(branch_uid)
        .bind(repo_uid)
        .bind(branch_name)
        .bind(message)
        .bind(author_name)
        .bind(author_email)
        .bind(commiter_name)
        .bind(commiter_email)
        .bind(timestamp)
        .bind(created_at)
        .fetch_one(pool)
        .await?;
        Ok(GitCommitModel {
            uid: row.get("uid"),
            sha: row.get("sha"),
            branch_uid: row.get("branch_uid"),
            repo_uid: row.get("repo_uid"),
            branch_name: row.get("branch_name"),
            message: row.get("message"),
            author_name: row.get("author_name"),
            author_email: row.get("author_email"),
            commiter_name: row.get("commiter_name"),
            commiter_email: row.get("commiter_email"),
            timestamp: row.get("timestamp"),
            created_at: row.get("created_at"),
        })
    }
    
    pub async fn get_by_repo_uid(
        pool: &PgPool,
        repo_uid: Uuid,
    ) -> Result<Vec<GitCommitModel>, Error> {
        let rows = sqlx::query(
            "SELECT * FROM git_commit WHERE repo_uid = $1",
        )
            .bind(repo_uid)
            .fetch_all(pool)
            .await?;
        Ok(rows.iter().map(|r|{
            GitCommitModel {
                uid: r.get("uid"),
                sha: r.get("sha"),
                branch_uid: r.get("branch_uid"),
                repo_uid: r.get("repo_uid"),
                branch_name: r.get("branch_name"),
                message: r.get("message"),
                author_name: r.get("author_name"),
                author_email: r.get("author_email"),
                commiter_name: r.get("commiter_name"),
                commiter_email: r.get("commiter_email"),
                timestamp: r.get("timestamp"),
                created_at: r.get("created_at"),
            }
        }).collect())
    }
    
    pub async fn get_by_sha(pool: &PgPool, sha: &str) -> AppResult<Option<GitCommitModel>> {
        let row = sqlx::query("SELECT * FROM git_commit WHERE sha = $1")
        .bind(sha)
        .fetch_optional(pool)
        .await?;
        Ok(row.map(|r| GitCommitModel {
            uid: r.get("uid"),
            sha: r.get("sha"),
            branch_uid: r.get("branch_uid"),
            repo_uid: r.get("repo_uid"),
            branch_name: r.get("branch_name"),
            message: r.get("message"),
            author_name: r.get("author_name"),
            author_email: r.get("author_email"),
            commiter_name: r.get("commiter_name"),
            commiter_email: r.get("commiter_email"),
            timestamp: r.get("timestamp"),
            created_at: r.get("created_at"),
        }))
    }
    
    

    pub async fn get_by_uid(pool: &PgPool, uid: Uuid) -> Result<Option<GitCommitModel>, Error> {
        let row = sqlx::query(
            r#"
        SELECT * FROM git_commit
        WHERE uid = $1
        "#,
        )
        .bind(uid)
        .fetch_optional(pool)
        .await?;
        Ok(row.map(|r| GitCommitModel {
            uid: r.get("uid"),
            sha: r.get("sha"),
            branch_uid: r.get("branch_uid"),
            repo_uid: r.get("repo_uid"),
            branch_name: r.get("branch_name"),
            message: r.get("message"),
            author_name: r.get("author_name"),
            author_email: r.get("author_email"),
            commiter_name: r.get("commiter_name"),
            commiter_email: r.get("commiter_email"),
            timestamp: r.get("timestamp"),
            created_at: r.get("created_at"),
        }))
    }

    pub async fn get_by_branch_uid(
        pool: &PgPool,
        branch_uid: Uuid,
    ) -> Result<Vec<GitCommitModel>, Error> {
        let rows = sqlx::query(
            r#"
        SELECT * FROM git_commit
        WHERE branch_uid = $1
        ORDER BY timestamp DESC
        "#,
        )
        .bind(branch_uid)
        .fetch_all(pool)
        .await?;
        let commits = rows
            .into_iter()
            .map(|r| GitCommitModel {
                uid: r.get("uid"),
                sha: r.get("sha"),
                branch_uid: r.get("branch_uid"),
                repo_uid: r.get("repo_uid"),
                branch_name: r.get("branch_name"),
                message: r.get("message"),
                author_name: r.get("author_name"),
                author_email: r.get("author_email"),
                commiter_name: r.get("commiter_name"),
                commiter_email: r.get("commiter_email"),
                timestamp: r.get("timestamp"),
                created_at: r.get("created_at"),
            })
            .collect();
        Ok(commits)
    }

    pub async fn update_message(
        pool: &PgPool,
        uid: Uuid,
        new_message: &str,
    ) -> Result<Option<GitCommitModel>, Error> {
        let row = sqlx::query(
            r#"
        UPDATE git_commit
        SET message = $1
        WHERE uid = $2
        RETURNING *
        "#,
        )
        .bind(new_message)
        .bind(uid)
        .fetch_optional(pool)
        .await?;
        Ok(row.map(|r| GitCommitModel {
            uid: r.get("uid"),
            sha: r.get("sha"),
            branch_uid: r.get("branch_uid"),
            repo_uid: r.get("repo_uid"),
            branch_name: r.get("branch_name"),
            message: r.get("message"),
            author_name: r.get("author_name"),
            author_email: r.get("author_email"),
            commiter_name: r.get("commiter_name"),
            commiter_email: r.get("commiter_email"),
            timestamp: r.get("timestamp"),
            created_at: r.get("created_at"),
        }))
    }

    pub async fn delete(pool: &PgPool, uid: Uuid) -> Result<Option<GitCommitModel>, Error> {
        let row = sqlx::query(
            r#"
        DELETE FROM git_commit
        WHERE uid = $1
        RETURNING *
        "#,
        )
        .bind(uid)
        .fetch_optional(pool)
        .await?;
        Ok(row.map(|r| GitCommitModel {
            uid: r.get("uid"),
            sha: r.get("sha"),
            branch_uid: r.get("branch_uid"),
            repo_uid: r.get("repo_uid"),
            branch_name: r.get("branch_name"),
            message: r.get("message"),
            author_name: r.get("author_name"),
            author_email: r.get("author_email"),
            commiter_name: r.get("commiter_name"),
            commiter_email: r.get("commiter_email"),
            timestamp: r.get("timestamp"),
            created_at: r.get("created_at"),
        }))
    }
}
