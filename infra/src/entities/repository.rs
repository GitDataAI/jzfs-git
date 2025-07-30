use crate::error::AppResult;
use chrono::Local;
use serde::{Deserialize, Serialize};
use sqlx::{Decode, Encode, Error, FromRow, PgPool, Row};
use std::path::PathBuf;
use uuid::Uuid;

#[derive(Deserialize, Serialize, Encode, Decode, FromRow,Clone)]
pub struct RepositoryModel {
    #[sqlx(primary_key)]
    pub uid: Uuid,
    pub name: String,
    pub owner: Uuid,
    pub description: String,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
    pub deleted_at: Option<chrono::NaiveDateTime>,
}

impl RepositoryModel {
    pub fn to_path(&self) -> PathBuf {
        PathBuf::from(format!("{}/{}", self.owner, self.name))
    }

    pub async fn create(
        pool: &PgPool,
        name: &str,
        owner: Uuid,
        description: &str,
    ) -> Result<RepositoryModel, Error> {
        let uid = Uuid::new_v4();
        let now = Local::now().naive_local();
        let row = sqlx::query(
            r#"
        INSERT INTO repository (uid, name, owner, description, created_at, updated_at, deleted_at)
        VALUES ($1, $2, $3, $4, $5, $6, $7)
        RETURNING *
        "#,
        )
        .bind(uid)
        .bind(name)
        .bind(owner)
        .bind(description)
        .bind(now)
        .bind(now)
        .bind(None::<chrono::NaiveDateTime>)
        .fetch_one(pool)
        .await?;
        Ok(RepositoryModel {
            uid: row.get("uid"),
            name: row.get("name"),
            owner: row.get("owner"),
            description: row.get("description"),
            created_at: row.get("created_at"),
            updated_at: row.get("updated_at"),
            deleted_at: row.get("deleted_at"),
        })
    }

    pub async fn get_by_uid(pool: &PgPool, uid: Uuid) -> Result<Option<RepositoryModel>, Error> {
        let row = sqlx::query(
            r#"
        SELECT * FROM repository
        WHERE uid = $1 AND deleted_at IS NULL
        "#,
        )
        .bind(uid)
        .fetch_optional(pool)
        .await?;
        Ok(row.map(|r| RepositoryModel {
            uid: r.get("uid"),
            name: r.get("name"),
            owner: r.get("owner"),
            description: r.get("description"),
            created_at: r.get("created_at"),
            updated_at: r.get("updated_at"),
            deleted_at: r.get("deleted_at"),
        }))
    }

    pub async fn get_by_owner(pool: &PgPool, owner: Uuid) -> Result<Vec<RepositoryModel>, Error> {
        let rows = sqlx::query(
            r#"
        SELECT * FROM repository
        WHERE owner = $1 AND deleted_at IS NULL
        "#,
        )
        .bind(owner)
        .fetch_all(pool)
        .await?;
        let repos = rows
            .into_iter()
            .map(|r| RepositoryModel {
                uid: r.get("uid"),
                name: r.get("name"),
                owner: r.get("owner"),
                description: r.get("description"),
                created_at: r.get("created_at"),
                updated_at: r.get("updated_at"),
                deleted_at: r.get("deleted_at"),
            })
            .collect();
        Ok(repos)
    }

    pub async fn update(
        pool: &PgPool,
        uid: Uuid,
        name: Option<&str>,
        description: Option<&str>,
    ) -> Result<Option<RepositoryModel>, Error> {
        let now = Local::now().naive_local();
        let row = sqlx::query(
            r#"
        UPDATE repository
        SET name = COALESCE($1, name),
            description = COALESCE($2, description),
            updated_at = $3
        WHERE uid = $4 AND deleted_at IS NULL
        RETURNING *
        "#,
        )
        .bind(name)
        .bind(description)
        .bind(now)
        .bind(uid)
        .fetch_optional(pool)
        .await?;
        Ok(row.map(|r| RepositoryModel {
            uid: r.get("uid"),
            name: r.get("name"),
            owner: r.get("owner"),
            description: r.get("description"),
            created_at: r.get("created_at"),
            updated_at: r.get("updated_at"),
            deleted_at: r.get("deleted_at"),
        }))
    }

    pub async fn delete(pool: &PgPool, uid: Uuid) -> Result<Option<RepositoryModel>, Error> {
        let now = Local::now().naive_local();
        let row = sqlx::query(
            r#"
        UPDATE repository
        SET deleted_at = $1
        WHERE uid = $2 AND deleted_at IS NULL
        RETURNING *
        "#,
        )
        .bind(now)
        .bind(uid)
        .fetch_optional(pool)
        .await?;
        Ok(row.map(|r| RepositoryModel {
            uid: r.get("uid"),
            name: r.get("name"),
            owner: r.get("owner"),
            description: r.get("description"),
            created_at: r.get("created_at"),
            updated_at: r.get("updated_at"),
            deleted_at: r.get("deleted_at"),
        }))
    }

    pub async fn list_all(pool: &PgPool) -> Result<Vec<RepositoryModel>, Error> {
        let rows = sqlx::query(
            r#"
        SELECT * FROM repository
        WHERE deleted_at IS NULL
        ORDER BY created_at DESC
        "#,
        )
        .fetch_all(pool)
        .await?;
        let repos = rows
            .into_iter()
            .map(|r| RepositoryModel {
                uid: r.get("uid"),
                name: r.get("name"),
                owner: r.get("owner"),
                description: r.get("description"),
                created_at: r.get("created_at"),
                updated_at: r.get("updated_at"),
                deleted_at: r.get("deleted_at"),
            })
            .collect();
        Ok(repos)
    }

    pub async fn repository_find_by_owner_name_and_repo_name(
        pool: &PgPool,
        owner: String,
        repo_name: String,
    ) -> AppResult<Option<RepositoryModel>> {
        let rows = sqlx::query(
            r#"
               SELECT r.* FROM repository r
                JOIN users u ON r.owner = u.uid
                WHERE r.name = $1 AND u.username = $2
                LIMIT 1
                "#,
        )
        .bind(repo_name)
        .bind(owner)
        .fetch_optional(pool)
        .await?;
        let repos = rows.map(|r| RepositoryModel {
            uid: r.get("uid"),
            name: r.get("name"),
            owner: r.get("owner"),
            description: r.get("description"),
            created_at: r.get("created_at"),
            updated_at: r.get("updated_at"),
            deleted_at: r.get("deleted_at"),
        });
        Ok(repos)
    }
}
