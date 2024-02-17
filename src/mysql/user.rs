use axum::async_trait;

use crate::{model, repository};

pub struct User {
    pub client: String,
}

impl User {
    pub fn new() -> Self {
        Self {
            client: format!("User"),
        }
    }
}

#[async_trait]
impl repository::User for User {
    async fn get_user(&self) -> model::Result<model::User> {
        println!("MySQL: {:#?}", self.client);
        let result = model::User {
            id: 1,
            name: format!("Sarawuth Pimsai"),
        };
        Ok(result)
    }
}
