use crate::{models::word_pair::WordPair, repositories::repository::Repository};
use async_trait::async_trait;
use sqlx::{Error, postgres::PgPool};

#[async_trait]
pub trait IWordPairRepository: Repository<Item = WordPair, Error = Error> {
    async fn select_by_user_id(&self, user_id: &i32) -> Result<Vec<Self::Item>, Self::Error>;
}

#[derive(Clone)]
pub struct WordPairRepository {
    pub db: PgPool,
}

#[async_trait]
impl Repository for WordPairRepository {
    type Item = WordPair;
    type Error = Error;

    async fn insert(&self, word_pair: &WordPair) -> Result<WordPair, Error> {
        let db_word_pair = sqlx::query_as!(
            WordPair,
            "INSERT INTO word_pairs (id, user_id, target_text, source_text, target_language, source_language) VALUES ($1, $2, $3, $4, $5, $6) RETURNING *", 
            &word_pair.id,
            &word_pair.user_id,
            &word_pair.target_text,
            &word_pair.source_text,
            &word_pair.target_language,
            &word_pair.source_language
        )
            .fetch_one(&self.db)
            .await?;

        Ok(db_word_pair)
    }
}

#[async_trait]
impl IWordPairRepository for WordPairRepository {
    async fn select_by_user_id(&self, user_id: &i32) -> Result<Vec<WordPair>, Error> {
        let db_word_pairs = sqlx::query_as!(
            WordPair,
            "SELECT id, user_id, target_text, source_text, target_language, source_language FROM word_pairs WHERE user_id = $1",
            &user_id
        )
            .fetch_all(&self.db)
            .await?;

        Ok(db_word_pairs)
    }
}
