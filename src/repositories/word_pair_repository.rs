use crate::models::word_pair::WordPair;
use sqlx::{Error, Row, postgres::PgPool};

pub struct WordPairRepository {
    pub db: PgPool,
}

impl WordPairRepository {
    pub async fn save_word_pair(&self, word_pair: WordPair) -> Result<(), Error> {
        sqlx::query(
            "INSERT INTO word_pairs (id, user_id, target_text, source_text, target_language, source_language) VALUES ($1, $2, $3, $4, $5)"
        )
            .bind(word_pair.id)
            .bind(word_pair.user_id)
            .bind(word_pair.target_text)
            .bind(word_pair.source_text)
            .bind(word_pair.target_language)
            .bind(word_pair.source_language)
            .execute(&self.db)
            .await?;

        Ok(())
    }

    pub async fn select_word_pairs(&self, user_id: i32) -> Result<Vec<WordPair>, Error> {
        let res = sqlx::query(
            "SELECT id, target_text, source_text, target_language, source_language FROM word_pairs WHERE user_id = $1"
        )
            .bind(user_id)
            .map(|row: sqlx::postgres::PgRow| {
                let word_pair = WordPair {
                    id: row.get("id"),
                    user_id: user_id,
                    target_text: row.get("target_text"),
                    source_text: row.get("source_text"),
                    target_language: row.get("target_language"),
                    source_language: row.get("source_language")
                };
                word_pair
            })
            .fetch_all(&self.db)
            .await?;

        Ok(res)
    }
}
