use crate::{models::user::User, repositories::repository::Repository};
use sqlx::{Error, postgres::PgPool};

pub struct UserRepository {
    pub db: PgPool,
}

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
