use axum::async_trait;

use crate::{model, repository};

pub struct Cart {
    client: String,
}

impl Cart {
    pub fn new() -> Self {
        Self {
            client: format!("Cart"),
        }
    }
}

#[async_trait]
impl repository::Cart for Cart {
    async fn get_cart(&self) -> model::Result<model::Cart> {
        println!("Persistence: {}", self.client);
        let cart = model::Cart {
            id: 1,
            product_id: 1,
        };
        Ok(cart)
    }
}
