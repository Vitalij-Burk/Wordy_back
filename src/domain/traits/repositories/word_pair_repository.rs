use async_trait::async_trait;

use crate::domain::{models::word_pair::WordPair, traits::repositories::repository::Repository};

#[async_trait]
pub trait IWordPairRepository: Repository<Item = WordPair> {
    async fn select_by_user_id(&self, user_id: &i32) -> Result<Vec<Self::Item>, Self::Error>;
}
