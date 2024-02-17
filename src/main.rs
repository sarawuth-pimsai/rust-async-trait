use atrait::{mysql, persistence, service};

#[tokio::main]
async fn main() {
    let repo = persistence::Cart::new();
    let user_repo = mysql::User::new();
    let cart_service = service::cart::Cart::new(repo, user_repo);
    let result = cart_service.cart().await.unwrap();
    let user = cart_service.user().await.unwrap();
    println!("{:#?}", result);
    println!("{:#?}", user);
    println!("Hello, world!");
}
