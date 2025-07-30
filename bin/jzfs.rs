use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use rsession::{RandKey, SessionBuilder};
use rsession::redis::RedisSessionStorage;
use api::ApiService;
use infra::config::pgsql::pgsql_client;
use infra::config::redis::redis_client;

#[tokio::main]
pub async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt().init();
    let redis_client = redis_client().await;
    let pgsql_client = pgsql_client().await?;
    let app = infra::App {
        db: pgsql_client,
        cache: redis_client.clone().into_storage(),
    };
    let session_storage = RedisSessionStorage::new(redis_client.single(), RandKey::RandomSha256(128))
        .set_prefix("session:");
    let session_builder = SessionBuilder::new()
        .rand_key(RandKey::RandomSha256(128));
    let port = std::env::var("PORT").unwrap_or("8080".to_string());
    let socket = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)), port.parse::<u16>().unwrap());
    let shell = shell::ssh::SSHHandle::new(app.clone());
    let api = ApiService {
        socket,
        app,
        storage: session_storage,
        session: session_builder,
    };
    shell.run().await;
    api.run().await;
    Ok(())
}