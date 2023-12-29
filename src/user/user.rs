use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;

#[derive(sqlx::Type, Debug, Serialize, Deserialize)]
#[sqlx(rename_all = "lowercase")]
pub enum UserRole {
    Admin,
    User,
}

#[derive(sqlx::Type, Debug, Serialize, Deserialize)]
pub enum UserStatus {
    #[sqlx(rename = "active")]
    Active,
    #[sqlx(rename = "inactive")]
    Inactive,
}

#[derive(FromRow, Debug, Serialize, Deserialize)]
pub struct User {
    pub id: i32,
    pub name: String,
    pub email: String,
    pub password: String,
    pub role: UserRole,
    pub status: UserStatus,
    pub created_at: DateTime<Utc>,
}
