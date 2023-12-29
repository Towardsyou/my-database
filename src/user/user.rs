use chrono::{DateTime, Utc};
use sqlx::prelude::FromRow;

#[derive(sqlx::Type, Debug)]
#[sqlx(rename_all = "lowercase")]
pub enum UserRole {
    Admin,
    User,
}

#[derive(sqlx::Type, Debug)]
pub enum UserStatus {
    #[sqlx(rename = "active")]
    Active,
    #[sqlx(rename = "inactive")]
    Inactive,
}

#[derive(FromRow, Debug)]
pub struct User {
    id: i32,
    name: String,
    email: String,
    password: String,
    role: UserRole,
    status: UserStatus,
    created_at: DateTime<Utc>,
}
