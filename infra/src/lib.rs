use sqlx::PgPool;
use crate::config::redis::RedisStorage;

pub mod entities;
pub mod error;
pub mod service;
pub mod types;

#[derive(Clone)]
pub struct App {
    pub db: PgPool,
    pub cache: RedisStorage,
}

pub mod config;
