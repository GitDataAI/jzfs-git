use std::net::SocketAddr;
use tracing::info;
use infra::App;
use crate::ssh::handle::SSHandle;

pub struct SSHServer {
    pub app: App,
}

impl SSHServer {
    pub fn new(app: App) -> Self {
        SSHServer {
            app,
        }
    }
}

impl russh::server::Server for SSHServer {
    type Handler = SSHandle;

    fn new_client(&mut self, addr: Option<SocketAddr>) -> Self::Handler {
        if let Some(addr) = addr {
            info!("New SSH connection from {}", addr);
        }
        SSHandle::new(self.app.clone())
    }
}