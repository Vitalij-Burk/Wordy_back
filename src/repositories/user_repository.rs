use crate::models::user::User;
use sqlx::{Error, postgres::PgPool};

pub struct UserRepository {
    pub db: PgPool,
}

impl UserRepository {
    pub async fn save_user(&self, user: User) -> Result<(), Error> {
        sqlx::query("INSERT INTO users (id, key, name) VALUES ($1, $2, $3)")
            .bind(user.id)
            .bind(user.key)
            .bind(user.name)
            .execute(&self.db)
            .await?;

        Ok(())
    }
}
