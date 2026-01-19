use std::borrow::Cow;

use thiserror::Error;
use tracing::error;

use crate::domain::{
    models::user::{CreateUser, User},
    traits::repositories::{repository::Repository, user_repository::IUserRepository},
};

#[derive(Clone)]
pub struct UserService<Repo> {
    repo: Repo,
}

#[derive(Error, Debug)]
pub enum UserServiceError {
    #[error("User with this key already exists: {0}")]
    KeyAlreadyExists(String),

    #[error("Invalid user key: {0}")]
    InvalidKey(String),

    #[error("Password is too easy: {0}")]
    InvalidPassword(String),

    #[error("User not found: `{0}`")]
    NotFound(String),

    #[error("Database error: `{0}`")]
    Database(#[from] sqlx::Error),
}

impl<Repo> UserService<Repo>
where
    Repo: Repository<Item = User, Error = sqlx::Error>,
{
    pub fn new(repo: Repo) -> Self {
        Self { repo: repo }
    }

    pub async fn create(&self, params: &CreateUser) -> Result<User, UserServiceError> {
        let user = User::new(&params.key, &params.name);

        let res = self
            .repo
            .insert(&user)
            .await
            .map_err(|error| match &error {
                sqlx::Error::Database(error) if error.code() == Some(Cow::Borrowed("23505")) => {
                    UserServiceError::KeyAlreadyExists(params.key.clone())
                }
                _ => {
                    error!("User DB error: {:?}", error);
                    UserServiceError::Database(error.into())
                }
            })?;

        Ok(res)
    }
}

impl<Repo> UserService<Repo>
where
    Repo: IUserRepository<Error = sqlx::Error>,
{
    pub async fn get_by_id(&self, id: &i32) -> Result<User, UserServiceError> {
        let res = self.repo.select_by_id(id).await.map_err(|error| {
            error!("User DB error: {}", error);
            error
        })?;

        Ok(res)
    }

    pub async fn get_by_key(&self, key: &str) -> Result<User, UserServiceError> {
        let res = self.repo.select_by_key(key).await.map_err(|error| {
            error!("User DB error: {}", error);
            error
        })?;

        Ok(res)
    }
}

#[cfg(test)]
mod tests {
    use async_trait::async_trait;

    use super::*;

    struct TestUserRepository {
        _db: i32,
    }

    #[async_trait]
    impl Repository for TestUserRepository {
        type Pool = i32;
        type Item = User;
        type Error = sqlx::Error;

        fn new(db: i32) -> Self {
            Self { _db: db }
        }

        async fn insert(&self, item: &Self::Item) -> Result<Self::Item, Self::Error> {
            Ok(item.clone())
        }
    }

    #[tokio::test]
    async fn test_create() {
        let repo = TestUserRepository { _db: 12345 };

        let user_service = UserService::new(repo);

        let test_user = CreateUser {
            key: "fsdfsf".to_string(),
            name: "Me".to_string(),
        };

        let res = user_service.create(&test_user).await.unwrap();

        assert_eq!(res.name, test_user.name);
        assert_eq!(res.key, test_user.key);
    }
}
