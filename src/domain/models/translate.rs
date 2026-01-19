use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct ToTranslate {
    pub source_text: String,
    pub target_language: String,
    pub source_language: String,
}
