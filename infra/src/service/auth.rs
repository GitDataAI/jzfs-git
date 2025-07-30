use crate::App;
use crate::entities::users::UsersModel;
use crate::error::{AppError, AppResult};
use crate::types::session::AuthSession;
use serde::{Deserialize, Serialize};
use sha256::Sha256Digest;

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct AuthLoginParam {
    pub username: String,
    pub password: String,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct AuthSignUpParam {
    pub username: String,
    pub password: String,
    pub email: String,
}

impl App {
    pub async fn service_auth_login(&self, param: AuthLoginParam) -> AppResult<AuthSession> {
        let mut user = UsersModel::get_by_username(&self.db, &param.username).await?;
        if user.is_none() {
            user = UsersModel::get_by_email(&self.db, &param.username).await?;
        }
        let Some(user) = user else {
            return Err(AppError::Custom("Invalid username or password".to_string()));
        };
        if user.password != param.password.digest() {
            return Err(AppError::Custom("Password is incorrect".to_string()));
        };
        let session = AuthSession::from(user);
        Ok(session)
    }
    pub async fn service_auth_signup(&self, param: AuthSignUpParam) -> AppResult<AuthSession> {
        if UsersModel::get_by_email(&self.db, &param.email)
            .await?
            .is_some()
        {
            return Err(AppError::Custom("Email already exists".to_string()));
        }
        if UsersModel::get_by_username(&self.db, &param.username)
            .await?
            .is_some()
        {
            return Err(AppError::Custom("Username already exists".to_string()));
        }
        let models =
            UsersModel::create(&self.db, &param.username, &param.email, &param.password).await?;
        Ok(AuthSession::from(models))
    }
}
