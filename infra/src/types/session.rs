use crate::entities::users;
use crate::entities::users::UsersModel;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct AuthSession {
    pub uid: Uuid,
    pub username: String,
    pub email: String,
}

impl AuthSession {
    pub const KEY: &'static str = "auth_session";
}

impl From<users::UsersModel> for AuthSession {
    fn from(value: UsersModel) -> Self {
        AuthSession {
            uid: value.uid,
            username: value.username,
            email: value.email,
        }
    }
}

#[async_trait]
pub trait AuthSessionExt {
    async fn to_auth(&self) -> Option<AuthSession>;
    async fn set_auth_session(&self, session: AuthSession);
    async fn clear_auth_session(&self);
}

#[async_trait]
impl AuthSessionExt for rsession::Session {
    async fn to_auth(&self) -> Option<AuthSession> {
        self.get::<AuthSession>(AuthSession::KEY).ok()
    }
    async fn set_auth_session(&self, session: AuthSession) {
        self.set(AuthSession::KEY, session).ok();
    }
    async fn clear_auth_session(&self) {
        self.remove(AuthSession::KEY);
    }
}
