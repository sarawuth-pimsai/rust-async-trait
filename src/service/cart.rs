use crate::{model, repository};

pub struct Cart<R, UR> {
    pub repo: R,
    pub user_repo: UR,
}

impl<R, UR> Cart<R, UR> {
    pub fn new(repo: R, user_repo: UR) -> Self {
        Self { repo, user_repo }
    }
}

impl<R, UR> Cart<R, UR>
where
    R: repository::Cart,
    UR: repository::User,
{
    pub async fn cart(&self) -> model::Result<model::Cart> {
        let result = self.repo.get_cart().await?;
        Ok(result)
    }
    pub async fn user(&self) -> model::Result<model::User> {
        let result = self.user_repo.get_user().await?;
        Ok(result)
    }
}
