use axum::async_trait;

use crate::model;

#[async_trait]
pub trait User {
    async fn get_user(&self) -> model::Result<model::User>;
}
