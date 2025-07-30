use chrono::Local;
use serde::{Deserialize, Serialize};
use sqlx::{Decode, Encode, Error, FromRow, PgPool, Row};
use uuid::Uuid;

#[derive(Deserialize, Serialize, Encode, Decode, FromRow)]
pub struct GitBranchModel {
    #[sqlx(primary_key)]
    pub uid: Uuid,
    pub repo_uid: Uuid,
    pub name: String,
    pub head: String,
    pub timestamp: i64,
}

impl GitBranchModel {
    pub async fn create(
        pool: &PgPool,
        repo_uid: Uuid,
        name: &str,
        head: &str,
    ) -> Result<GitBranchModel, Error> {
        let uid = Uuid::new_v4();
        let timestamp = Local::now().timestamp();
        let row = sqlx::query(
            r#"
        INSERT INTO git_branch (uid, repo_uid, name, head, timestamp)
        VALUES ($1, $2, $3, $4, $5)
        RETURNING *
        "#,
        )
        .bind(uid)
        .bind(repo_uid)
        .bind(name)
        .bind(head)
        .bind(timestamp)
        .fetch_one(pool)
        .await?;
        Ok(GitBranchModel {
            uid: row.get("uid"),
            repo_uid: row.get("repo_uid"),
            name: row.get("name"),
            head: row.get("head"),
            timestamp: row.get("timestamp"),
        })
    }

    pub async fn get_by_uid(pool: &PgPool, uid: Uuid) -> Result<Option<GitBranchModel>, Error> {
        let row = sqlx::query(
            r#"
        SELECT * FROM git_branch
        WHERE uid = $1
        "#,
        )
        .bind(uid)
        .fetch_optional(pool)
        .await?;
        Ok(row.map(|r| GitBranchModel {
            uid: r.get("uid"),
            repo_uid: r.get("repo_uid"),
            name: r.get("name"),
            head: r.get("head"),
            timestamp: r.get("timestamp"),
        }))
    }

    pub async fn get_by_repo_uid(
        pool: &PgPool,
        repo_uid: Uuid,
    ) -> Result<Vec<GitBranchModel>, Error> {
        let rows = sqlx::query(
            r#"
        SELECT * FROM git_branch
        WHERE repo_uid = $1
        ORDER BY name
        "#,
        )
        .bind(repo_uid)
        .fetch_all(pool)
        .await?;
        let branches = rows
            .into_iter()
            .map(|r| GitBranchModel {
                uid: r.get("uid"),
                repo_uid: r.get("repo_uid"),
                name: r.get("name"),
                head: r.get("head"),
                timestamp: r.get("timestamp"),
            })
            .collect();
        Ok(branches)
    }

    pub async fn update(
        pool: &PgPool,
        uid: Uuid,
        name: Option<&str>,
        head: Option<&str>,
    ) -> Result<Option<GitBranchModel>, Error> {
        let timestamp = Local::now().timestamp();
        let row = sqlx::query(
            r#"
        UPDATE git_branch
        SET name = COALESCE($1, name),
            head = COALESCE($2, head),
            timestamp = $3
        WHERE uid = $4
        RETURNING *
        "#,
        )
        .bind(name)
        .bind(head)
        .bind(timestamp)
        .bind(uid)
        .fetch_optional(pool)
        .await?;
        Ok(row.map(|r| GitBranchModel {
            uid: r.get("uid"),
            repo_uid: r.get("repo_uid"),
            name: r.get("name"),
            head: r.get("head"),
            timestamp: r.get("timestamp"),
        }))
    }

    pub async fn delete(pool: &PgPool, uid: Uuid) -> Result<Option<GitBranchModel>, Error> {
        let branch = sqlx::query(
            r#"
        DELETE FROM git_branch
        WHERE uid = $1
        RETURNING *
        "#,
        )
        .bind(uid)
        .fetch_optional(pool)
        .await?;
        Ok(branch.map(|r| GitBranchModel {
            uid: r.get("uid"),
            repo_uid: r.get("repo_uid"),
            name: r.get("name"),
            head: r.get("head"),
            timestamp: r.get("timestamp"),
        }))
    }
}
