use heck::ToTitleCase;
use rand::Rng;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct CreateWordPair {
    pub target_text: String,
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
            target_text: target_text.to_string().to_title_case(),
            source_text: source_text.to_string().to_title_case(),
            target_language: target_language.to_string().to_lowercase(),
            source_language: source_language.to_string().to_lowercase(),
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize, sqlx::FromRow)]
pub struct WordPairEntity {
    pub id: i32,
    pub user_id: i32,

    pub target_text: String,
    pub source_text: String,
    pub target_language: String,
    pub source_language: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct WordPairDomain {
    pub id: i32,
    pub user_id: i32,

    pub target_text: String,
    pub source_text: String,
    pub target_language: String,
    pub source_language: String,
}

impl WordPairDomain {
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
            target_text: target_text.to_string().to_title_case(),
            source_text: source_text.to_string().to_title_case(),
            target_language: target_language.to_string().to_lowercase(),
            source_language: source_language.to_string().to_lowercase(),
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct WordPairDTO {
    pub id: i32,

    pub target_text: String,
    pub source_text: String,
    pub target_language: String,
    pub source_language: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CreateWordPairDTO {
    pub target_text: String,
    pub source_text: String,
    pub target_language: String,
    pub source_language: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum DeleteWordPairDTO {
    ById { id: i32 },
}
