use rand::Rng;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct CreateWordPair {
    pub source_text: String,
    pub target_language: String,
    pub source_language: String,
}

#[derive(Debug, Clone, Deserialize, Serialize, sqlx::FromRow)]
pub struct WordPair {
    pub id: i32,
    pub user_id: i32,

    pub target_text: String,
    pub source_text: String,
    pub target_language: String,
    pub source_language: String,
}

impl WordPair {
    pub fn new(
        user_id: &i32,
        target_text: &str,
        source_text: &str,
        target_language: &str,
        source_language: &str,
    ) -> Self {
        let id = rand::rng().random::<i32>();

        Self {
            id: id,
            user_id: *user_id,
            target_text: target_text.to_string(),
            source_text: source_text.to_string(),
            target_language: target_language.to_string(),
            source_language: source_language.to_string(),
        }
    }
}
