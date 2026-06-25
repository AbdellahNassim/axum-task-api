use argon2::{
    password_hash::{
        rand_core::OsRng,
        PasswordHasher,
        SaltString
    },
    Argon2
};

use chrono::Utc;
use uuid::Uuid;

use crate::{models::user::User, repositories::user_repository::UserRepository};

pub fn hash_password(password: &str) -> Result<String, String> {

    let salt = SaltString::generate(&mut OsRng);

    let hash = Argon2::default().hash_password(password.as_bytes(), &salt)
    .map_err(|e| e.to_string())?;
    Ok(hash.to_string())
}

pub async fn register_user(
    user_repo: &impl UserRepository,
    email: String,
    password: String,
) -> Result<User, String> {

    if user_repo.find_by_email(&email).await.map_err(|e| e.to_string())?.is_some() {
        return Err("User already exists".to_string());
    }
    let password_hash = hash_password(&password).map_err(|e| e.to_string())?;
    let user = User {
        id: Uuid::new_v4(),
        email,
        password_hash,
        created_at: Utc::now(),
    };
    user_repo.create(&user).await.map_err(|e| e.to_string())?;
    Ok(user)
}
