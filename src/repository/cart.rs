use axum::async_trait;

use crate::model;

#[async_trait]
pub trait Cart {
    async fn get_cart(&self) -> model::Result<model::Cart>;
}
