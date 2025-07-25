use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;
use uuid::Uuid;

#[derive(Debug, Serialize, FromRow)]
pub struct User {
    pub id: Uuid,
    pub name: String,
    pub hashed_password: String,
    pub created_at: NaiveDateTime,
}

#[derive(Debug, Deserialize)]
pub struct NewUser {
    pub name: String,
    pub password: String,
}

#[derive(Debug, Deserialize)]
pub struct LoginRequest {
    pub name: String,
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub user_id: Uuid,
    pub exp: usize,
}
