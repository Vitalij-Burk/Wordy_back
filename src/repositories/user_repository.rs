use crate::{models::user::User, repositories::repository::Repository};
use async_trait::async_trait;
use sqlx::{Error, postgres::PgPool};

#[async_trait]
pub trait IUserRepository: Repository<Item = User, Error = Error> {
    async fn select_by_id(&self, id: &i32) -> Result<Self::Item, Self::Error>;

    async fn select_by_key(&self, key: &str) -> Result<Self::Item, Self::Error>;
}

#[derive(Clone)]
pub struct UserRepository {
    pub db: PgPool,
}

#[async_trait]
impl Repository for UserRepository {
    type Item = User;
    type Error = Error;

    async fn insert(&self, user: &User) -> Result<User, Error> {
        let db_user = sqlx::query_as!(
            User,
            "INSERT INTO users (id, key, name) VALUES ($1, $2, $3) RETURNING *",
            &user.id,
            &user.key,
            &user.name
        )
        .fetch_one(&self.db)
        .await?;

        Ok(db_user)
    }
}

#[async_trait]
impl IUserRepository for UserRepository {
    async fn select_by_id(&self, id: &i32) -> Result<Self::Item, Self::Error> {
        let db_user = sqlx::query_as!(User, "SELECT id, key, name FROM users WHERE id=$1", id)
            .fetch_one(&self.db)
            .await?;

        Ok(db_user)
    }

    async fn select_by_key(&self, key: &str) -> Result<Self::Item, Self::Error> {
        let db_user = sqlx::query_as!(User, "SELECT id, key, name FROM users WHERE key=$1", key)
            .fetch_one(&self.db)
            .await?;

        Ok(db_user)
    }
}
