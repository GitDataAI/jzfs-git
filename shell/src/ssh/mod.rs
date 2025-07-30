use std::sync::Arc;
use russh::keys::PrivateKey;
use russh::keys::ssh_key::private::Ed25519Keypair;
use russh::server::{Config, Server};
use russh::{MethodSet, SshId};
use tracing::info;
use infra::App;
use crate::ssh::server::SSHServer;

pub mod handle;
pub mod server;

#[derive(Clone)]
pub struct SSHHandle {
    pub app: App,
}

impl SSHHandle {
    pub async fn run(&self) {
        let this = self.clone();
        tokio::spawn(async move{
            this.run_ssh().await.ok();
        });
    }
    pub fn new(app: App) -> Self {
        Self {
            app,
        }
    }
    pub async fn run_ssh(&self) -> anyhow::Result<()>{
        info!("SSH Starting...");
        let env = std::env::var("JZFS_ED25519").expect("ED25519 not set");
        let vec = hex::decode(env.as_bytes().to_vec())?;
        let key_bytes: [u8; 64] = vec.try_into().expect("Invalid key length");
        let key = Ed25519Keypair::from_bytes(&key_bytes)?;
        let mut config = Config::default();
        config.keys = vec![PrivateKey::from(key)];
        let version = format!("SSH-2.0-Gitdata {}", env!("CARGO_PKG_VERSION"));
        config.server_id = SshId::Standard(version);
        config.methods = MethodSet::all();
        config.maximum_packet_size = 65535;
        let mut server = SSHServer::new(self.app.clone());
        info!("Starting SSH server, with: ssh://0.0.0.0:30322");
        server.run_on_address(Arc::new(config), "0.0.0.0:30322").await?;
        Ok(())
    }
}