use crate::error::AppResult;
use deadpool_redis::Runtime;
use deadpool_redis::redis::{Cmd, FromRedisValue};

#[derive(Clone)]
pub enum RedisConfig {
    Cluster(deadpool_redis::cluster::Pool),
    Single(deadpool_redis::Pool),
}

pub async fn redis_client() -> RedisConfig {
    dotenv::dotenv().ok();
    let vars = dotenv::vars()
        .filter(|(k, _)| k.contains("REDIS_"))
        .map(|(k, v)| (k, v))
        .map(|x| x.1)
        .collect::<Vec<_>>();
    if vars.len() > 1 {
        let cur = deadpool_redis::cluster::Config::from_urls(vars);
        let pool = cur
            .builder()
            .unwrap()
            .runtime(Runtime::Tokio1)
            .build()
            .unwrap();
        RedisConfig::Cluster(pool)
    } else if vars.len() == 1 {
        let cur = deadpool_redis::Config::from_url(vars[0].clone());
        let pool = cur
            .builder()
            .unwrap()
            .runtime(Runtime::Tokio1)
            .build()
            .unwrap();
        return RedisConfig::Single(pool);
    } else {
        panic!("Redis config error");
    }
}

impl RedisConfig {
    pub fn into_storage( self) -> RedisStorage {
        RedisStorage {
            config: self,
        }
    }
    pub fn single(self) -> deadpool_redis::Pool {
        match self {
            RedisConfig::Single(pool) => pool,
            _ => panic!("Redis config error"),
        }
    }
}

#[derive(Clone)]
pub struct RedisStorage {
    pub config: RedisConfig,
}


impl RedisStorage {
    async fn exec<T>(&self, cmd: Cmd) -> AppResult<T>
    where
        T: FromRedisValue,
    {
        match &self.config {
            RedisConfig::Cluster(pool) => {
                let mut client = pool.get().await?;
                let result: T = cmd.query_async(&mut client).await?;
                Ok(result.into())
            }
            RedisConfig::Single(pool) => {
                let mut client = pool.get().await?;
                let result: T = cmd.query_async(&mut client).await?;
                Ok(result.into())
            }
        }
    }
}

impl RedisStorage {
    pub async fn get<T>(&self, key: &str) -> AppResult<Option<T>>
    where
        T: FromRedisValue,
    {
        self.exec(Cmd::get(key)).await
    }
    pub async fn set<T>(&self, key: &str, value: T) -> AppResult<()>
    where
        T: Into<String> + deadpool_redis::redis::ToRedisArgs,
    {
        self.exec::<()>(Cmd::set(key, value)).await?;
        Ok(())
    }
    pub async fn del(&self, key: &str) -> AppResult<()> {
        self.exec::<()>(Cmd::del(key)).await?;
        Ok(())
    }
    pub async fn exists(&self, key: &str) -> AppResult<bool> {
        self.exec(Cmd::exists(key)).await
    }
    pub async fn expire(&self, key: &str, seconds: i64) -> AppResult<()> {
        self.exec::<()>(Cmd::expire(key, seconds)).await?;
        Ok(())
    }
    pub async fn ttl(&self, key: &str) -> AppResult<i64> {
        self.exec(Cmd::ttl(key)).await
    }
}
