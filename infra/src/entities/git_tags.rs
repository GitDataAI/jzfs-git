use chrono::{NaiveDateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{Decode, Encode, Error, FromRow, PgPool, Row};
use uuid::Uuid;

#[derive(Deserialize, Serialize, Encode, Decode, FromRow)]
pub struct GitTags {
    #[sqlx(primary_key)]
    pub uid: Uuid,
    pub repo_uid: Uuid,
    pub name: String,
    pub sha: String,
    pub created_at: NaiveDateTime,
}

impl GitTags {
    pub async fn create(
        pool: &PgPool,
        repo_uid: Uuid,
        name: &str,
        sha: &str,
    ) -> Result<GitTags, Error> {
        let uid = Uuid::new_v4();
        let created_at = Utc::now().naive_utc();
        let row = sqlx::query(
            r#"
        INSERT INTO git_tags (uid, repo_uid, name, sha, created_at)
        VALUES ($1, $2, $3, $4, $5)
        RETURNING *
        "#,
        )
        .bind(uid)
        .bind(repo_uid)
        .bind(name)
        .bind(sha)
        .bind(created_at)
        .fetch_one(pool)
        .await?;
        Ok(GitTags {
            uid: row.get("uid"),
            repo_uid: row.get("repo_uid"),
            name: row.get("name"),
            sha: row.get("sha"),
            created_at: row.get("created_at"),
        })
    }

    pub async fn get_by_uid(pool: &PgPool, uid: Uuid) -> Result<Option<GitTags>, Error> {
        let row = sqlx::query(
            r#"
        SELECT * FROM git_tags
        WHERE uid = $1
        "#,
        )
        .bind(uid)
        .fetch_optional(pool)
        .await?;
        Ok(row.map(|r| GitTags {
            uid: r.get("uid"),
            repo_uid: r.get("repo_uid"),
            name: r.get("name"),
            sha: r.get("sha"),
            created_at: r.get("created_at"),
        }))
    }

    pub async fn get_by_repo_uid(pool: &PgPool, repo_uid: Uuid) -> Result<Vec<GitTags>, Error> {
        let rows = sqlx::query(
            r#"
        SELECT * FROM git_tags
        WHERE repo_uid = $1
        ORDER BY created_at DESC
        "#,
        )
        .bind(repo_uid)
        .fetch_all(pool)
        .await?;
        let tags = rows
            .into_iter()
            .map(|r| GitTags {
                uid: r.get("uid"),
                repo_uid: r.get("repo_uid"),
                name: r.get("name"),
                sha: r.get("sha"),
                created_at: r.get("created_at"),
            })
            .collect();
        Ok(tags)
    }

    pub async fn update(
        pool: &PgPool,
        uid: Uuid,
        name: Option<&str>,
        sha: Option<&str>,
    ) -> Result<Option<GitTags>, Error> {
        let row = sqlx::query(
            r#"
        UPDATE git_tags
        SET name = COALESCE($1, name),
            sha = COALESCE($2, sha)
        WHERE uid = $3
        RETURNING *
        "#,
        )
        .bind(name)
        .bind(sha)
        .bind(uid)
        .fetch_optional(pool)
        .await?;
        Ok(row.map(|r| GitTags {
            uid: r.get("uid"),
            repo_uid: r.get("repo_uid"),
            name: r.get("name"),
            sha: r.get("sha"),
            created_at: r.get("created_at"),
        }))
    }

    pub async fn delete(pool: &PgPool, uid: Uuid) -> Result<Option<GitTags>, Error> {
        let row = sqlx::query(
            r#"
        DELETE FROM git_tags
        WHERE uid = $1
        RETURNING *
        "#,
        )
        .bind(uid)
        .fetch_optional(pool)
        .await?;
        Ok(row.map(|r| GitTags {
            uid: r.get("uid"),
            repo_uid: r.get("repo_uid"),
            name: r.get("name"),
            sha: r.get("sha"),
            created_at: r.get("created_at"),
        }))
    }
}
