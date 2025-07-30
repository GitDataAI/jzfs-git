use crate::auth::context::auth_context;
use crate::auth::login::{auth_login, auth_logout};
use crate::auth::register::auth_register;
use actix_web::web;
use actix_web::web::{get, post, scope, Data};
use infra::App;
use rsession::framework::actix::ActixSessionMiddleware;
use rsession::redis::RedisSessionStorage;
use rsession::SessionBuilder;
use std::net::SocketAddr;
use actix_files::Files;
use tracing::{error, info};
use crate::repo::branch::repo_branch;
use crate::repo::dash::repo_dash;
use crate::repo::tree::repo_tree;
use crate::repo::commits::repo_commits;
use crate::repo::init::repo_init;
use crate::repo::list::repo_list;

#[derive(Clone)]
pub struct ApiService {
    pub socket: SocketAddr,
    pub app: App,
    pub storage: RedisSessionStorage,
    pub session: SessionBuilder,
}
impl ApiService {
    pub async fn run(self) {
        info!("Api Server start with: {}", self.socket);
        let app = actix_web::HttpServer::new(move || {
            actix_web::App::new()
                .wrap(actix_web::middleware::Logger::default())
                .wrap(actix_web::middleware::Compress::default())
                .wrap(ActixSessionMiddleware::new(
                    self.session.clone(),
                    self.storage.clone(),
                ))
                .app_data(Data::new(self.app.clone()))
                .configure(Self::router)
        })
        .bind(self.socket)
        .expect("Can not bind socket")
        .run();
        if let Err(e) = app.await {
            error!("Api service error:{}", e);
        }
    }
    pub fn router(cfg: &mut web::ServiceConfig) {
        cfg
            .service(
            scope("/api")
                .service(
                scope("/auth")
                    .route("/login", post().to(auth_login))
                    .route("/register", post().to(auth_register))
                    .route("/logout", post().to(auth_logout))
                    .route("/context", post().to(auth_context)),
            )
                .service(
                    scope("/repo")
                        .route("/init", post().to(repo_init))
                        .route("/list", get().to(repo_list))
                        .service(
                    scope("/{owner}/{repo}")
                        .route("",get().to(repo_dash))
                        .route("/tree/{path:.*}",get().to(repo_tree))
                        .route("/commits",get().to(repo_commits))
                        .route("/branches", get().to(repo_branch))
                        )
                )
                .service(
                    scope("/git")
                        .configure(shell::http::git_route)
                )
        )
            .route("{tail:.*}",get().to(dist::dist))
        ;
    }
}
mod auth;
mod repo;
mod error;
mod dist;