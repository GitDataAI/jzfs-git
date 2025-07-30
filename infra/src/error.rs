use std::fmt::Display;

pub type AppResult<T> = Result<T, AppError>;

pub enum AppError {
    Anyhow(anyhow::Error),
    UnAuth,
    Io(std::io::Error),
    Serde(serde_json::Error),
    Database(sqlx::Error),
    Custom(String),
    RedisPool(deadpool_redis::PoolError),
    Redis(deadpool_redis::redis::RedisError),
}
impl From<anyhow::Error> for AppError {
    fn from(err: anyhow::Error) -> Self {
        AppError::Anyhow(err)
    }
}
impl From<std::io::Error> for AppError {
    fn from(err: std::io::Error) -> Self {
        AppError::Io(err)
    }
}
impl From<serde_json::Error> for AppError {
    fn from(err: serde_json::Error) -> Self {
        AppError::Serde(err)
    }
}
impl From<sqlx::Error> for AppError {
    fn from(err: sqlx::Error) -> Self {
        AppError::Database(err)
    }
}
impl From<String> for AppError {
    fn from(err: String) -> Self {
        AppError::Custom(err)
    }
}

impl From<deadpool_redis::PoolError> for AppError {
    fn from(err: deadpool_redis::PoolError) -> Self {
        AppError::RedisPool(err)
    }
}
impl From<deadpool_redis::redis::RedisError> for AppError {
    fn from(err: deadpool_redis::redis::RedisError) -> Self {
        AppError::Redis(err)
    }
}

impl Display for AppError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            AppError::Anyhow(err) => err.to_string(),
            AppError::UnAuth => "UnAuth".to_string(),
            AppError::Io(err) => err.to_string(),
            AppError::Serde(err) => err.to_string(),
            AppError::Database(err) => err.to_string(),
            AppError::Custom(err) => err.to_string(),
            AppError::RedisPool(err) => err.to_string(),
            AppError::Redis(err) => err.to_string(),
        };
        write!(f, "{}", str)
    }
}
