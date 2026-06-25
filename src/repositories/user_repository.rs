use async_trait::async_trait;

use crate::models::user::User;

#[async_trait]
pub trait UserRepository {
    async fn create(&self, user: &User) -> Result<(), sqlx::Error>;

    async fn find_by_email(&self, email: &str) -> Result<Option<User>, sqlx::Error>;
}