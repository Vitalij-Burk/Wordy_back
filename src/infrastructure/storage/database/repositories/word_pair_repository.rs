use crate::domain::{
    models::word_pair::WordPair,
    traits::repositories::{repository::Repository, word_pair_repository::IWordPairRepository},
};
use async_trait::async_trait;
use sqlx::{Error, postgres::PgPool};

#[derive(Clone)]
pub struct WordPairPostgresRepository {
    db: PgPool,
}

#[async_trait]
impl Repository for WordPairPostgresRepository {
    type Pool = PgPool;
    type Item = WordPair;
    type Error = Error;

    fn new(db: PgPool) -> Self {
        Self { db: db }
    }

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
impl IWordPairRepository for WordPairPostgresRepository {
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
