use chrono::Local;
use serde::{Deserialize, Serialize};
use sha256::Sha256Digest;
use sqlx::{Decode, Encode, Error, FromRow, PgPool, Row};
use uuid::Uuid;

#[derive(Deserialize, Serialize, Encode, Decode, FromRow,Debug)]
pub struct UsersModel {
    #[sqlx(primary_key)]
    pub uid: Uuid,
    pub username: String,
    pub password: String,
    pub email: String,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
    pub deleted_at: Option<chrono::NaiveDateTime>,
}

impl UsersModel {
    pub fn new(username: String, password: String, email: String) -> Self {
        let now = Local::now().naive_local();
        Self {
            uid: Uuid::new_v4(),
            username,
            password: password.digest(),
            email,
            created_at: now,
            updated_at: now,
            deleted_at: None,
        }
    }
    pub fn verify_password(&self, password: &str) -> bool {
        self.password == password.digest()
    }

    pub async fn create(
        pool: &PgPool,
        username: &str,
        password: &str,
        email: &str,
    ) -> Result<UsersModel, Error> {
        let uid = Uuid::new_v4();
        let now = Local::now().naive_local();
        let row = sqlx::query(
            r#"
        INSERT INTO users (uid, username, password, email, created_at, updated_at, deleted_at)
        VALUES ($1, $2, $3, $4, $5, $6, $7)
        RETURNING *
        "#,
        )
        .bind(uid)
        .bind(username)
        .bind(password.digest())
        .bind(email)
        .bind(now)
        .bind(now)
        .bind(None::<chrono::NaiveDateTime>)
        .fetch_one(pool)
        .await?;
        Ok(UsersModel {
            uid: row.get("uid"),
            username: row.get("username"),
            password: row.get("password"),
            email: row.get("email"),
            created_at: row.get("created_at"),
            updated_at: row.get("updated_at"),
            deleted_at: row.get("deleted_at"),
        })
    }

    pub async fn get_by_uid(pool: &PgPool, uid: Uuid) -> Result<Option<UsersModel>, Error> {
        let row = sqlx::query(
            r#"
        SELECT * FROM users
        WHERE uid = $1 AND deleted_at IS NULL
        "#,
        )
        .bind(uid)
        .fetch_optional(pool)
        .await?;
        Ok(row.map(|r| UsersModel {
            uid: r.get("uid"),
            username: r.get("username"),
            password: r.get("password"),
            email: r.get("email"),
            created_at: r.get("created_at"),
            updated_at: r.get("updated_at"),
            deleted_at: r.get("deleted_at"),
        }))
    }
    pub async fn get_by_username(
        pool: &PgPool,
        username: &str,
    ) -> Result<Option<UsersModel>, Error> {
        let row = sqlx::query(
            r#"
        SELECT * FROM users
        WHERE username = $1
        "#,
        )
        .bind(username)
        .fetch_optional(pool)
        .await?;
        Ok(row.map(|r| UsersModel {
            uid: r.get("uid"),
            username: r.get("username"),
            password: r.get("password"),
            email: r.get("email"),
            created_at: r.get("created_at"),
            updated_at: r.get("updated_at"),
            deleted_at: r.get("deleted_at"),
        }))
    }
    pub async fn get_by_email(
        pool: &PgPool,
        email: &str,
    ) -> Result<Option<UsersModel>, sqlx::Error> {
        let row = sqlx::query(
            r#"
        SELECT * FROM users
        WHERE email = $1
        "#,
        )
        .bind(email)
        .fetch_optional(pool)
        .await?;
        Ok(row.map(|r| UsersModel {
            uid: r.get("uid"),
            username: r.get("username"),
            password: r.get("password"),
            email: r.get("email"),
            created_at: r.get("created_at"),
            updated_at: r.get("updated_at"),
            deleted_at: r.get("deleted_at"),
        }))
    }

    pub async fn update(
        pool: &PgPool,
        uid: Uuid,
        username: Option<&str>,
        email: Option<&str>,
    ) -> Result<Option<UsersModel>, Error> {
        let now = Local::now().naive_local();
        let row = sqlx::query(
            r#"
        UPDATE users
        SET username = COALESCE($1, username),
            email = COALESCE($2, email),
            updated_at = $3
        WHERE uid = $4 AND deleted_at IS NULL
        RETURNING *
        "#,
        )
        .bind(username)
        .bind(email)
        .bind(now)
        .bind(uid)
        .fetch_optional(pool)
        .await?;
        Ok(row.map(|r| UsersModel {
            uid: r.get("uid"),
            username: r.get("username"),
            password: r.get("password"),
            email: r.get("email"),
            created_at: r.get("created_at"),
            updated_at: r.get("updated_at"),
            deleted_at: r.get("deleted_at"),
        }))
    }

    pub async fn delete(pool: &PgPool, uid: Uuid) -> Result<Option<UsersModel>, Error> {
        let now = Local::now().naive_local();
        let row = sqlx::query(
            r#"
        UPDATE users
        SET deleted_at = $1
        WHERE uid = $2 AND deleted_at IS NULL
        RETURNING *
        "#,
        )
        .bind(now)
        .bind(uid)
        .fetch_optional(pool)
        .await?;
        Ok(row.map(|r| UsersModel {
            uid: r.get("uid"),
            username: r.get("username"),
            password: r.get("password"),
            email: r.get("email"),
            created_at: r.get("created_at"),
            updated_at: r.get("updated_at"),
            deleted_at: r.get("deleted_at"),
        }))
    }

    pub async fn list(pool: &PgPool) -> Result<Vec<UsersModel>, Error> {
        let rows = sqlx::query(
            r#"
        SELECT * FROM users
        WHERE deleted_at IS NULL
        "#,
        )
        .fetch_all(pool)
        .await?;
        let users = rows
            .into_iter()
            .map(|r| UsersModel {
                uid: r.get("uid"),
                username: r.get("username"),
                password: r.get("password"),
                email: r.get("email"),
                created_at: r.get("created_at"),
                updated_at: r.get("updated_at"),
                deleted_at: r.get("deleted_at"),
            })
            .collect();
        Ok(users)
    }
}
