use heck::ToTitleCase;
use rand::Rng;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct CreateUser {
    pub key: String,
    pub name: String,
}

#[derive(Debug, Clone, Deserialize, Serialize, sqlx::FromRow)]
pub struct User {
    pub id: i32,

    pub key: String,
    pub name: String,
}

impl User {
    pub fn new(key: &str, name: &str) -> Self {
        let id = rand::rng().random::<i32>();

        Self {
            id: id,
            key: key.to_string(),
            name: name.to_string().to_title_case(),
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize, sqlx::FromRow)]
pub struct UserEntity {
    pub id: i32,

    pub key: String,
    pub name: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct UserD {
    pub id: i32,

    pub key: String,
    pub name: String,
}

impl UserD {
    pub fn new(key: &str, name: &str) -> Self {
        let id = rand::rng().random::<i32>();

        Self {
            id: id,
            key: key.to_string(),
            name: name.to_string().to_title_case(),
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct UserDTO {
    pub id: i32,

    pub key: String,
    pub name: String
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CreateUserDTO {
    pub key: String,
    pub name: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct UpdateUserDTO {
    pub key: Option<String>,
    pub name: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum DeleteUserDTO {
    ById { id: i32 },
    ByKey { key: String },
}
