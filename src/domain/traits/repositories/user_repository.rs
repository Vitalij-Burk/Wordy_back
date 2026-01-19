use async_trait::async_trait;

use crate::domain::{models::user::User, traits::repositories::repository::Repository};

#[async_trait]
pub trait IUserRepository: Repository<Item = User> {
    async fn select_by_id(&self, id: &i32) -> Result<Self::Item, Self::Error>;

    async fn select_by_key(&self, key: &str) -> Result<Self::Item, Self::Error>;
}
