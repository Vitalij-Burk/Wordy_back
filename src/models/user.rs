use rand::Rng;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct CreateUser {
    pub key: String,
    pub name: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct User {
    pub id: i32,

    pub key: String,
    pub name: String,
    //word_pairs: HashSet<WordPair>,
}

impl User {
    pub fn new(key: &str, name: &str) -> Self {
        let id = rand::rng().random::<i32>();

        Self {
            id: id,
            key: key.to_string(),
            name: name.to_string(),
        }
    }
}
