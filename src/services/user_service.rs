use thiserror::Error;

use crate::{
    models::user::{CreateUser, User},
    repositories::repository::Repository,
};

pub struct UserService<T> {
    repo: T,
}

#[derive(Error, Debug)]
pub enum UserServiceError {
    #[error("Database error")]
    Database(#[from] sqlx::Error),
}

impl<T> UserService<T>
where
    T: Repository<Item = User, Error = sqlx::Error>,
{
    pub fn new(repo: T) -> Self {
        Self { repo: repo }
    }

    pub async fn create(&self, params: &CreateUser) -> Result<User, UserServiceError> {
        let user = User::new(&params.key, &params.name);

        let res = self.repo.insert(&user).await?;

        Ok(res)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestUserRepository {
        _db: i32,
    }

    impl Repository for TestUserRepository {
        type Item = User;
        type Error = sqlx::Error;

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
            name: "me".to_string(),
        };

        let res = user_service.create(&test_user).await.unwrap();

        assert_eq!(res.name, test_user.name);
        assert_eq!(res.key, test_user.key);
    }
}
