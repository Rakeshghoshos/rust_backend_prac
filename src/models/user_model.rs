use chrono::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize ,clone ,copy , PartialEq)]
#[sqlx(type_name = "user_role", rename_all = "lowercase")]
pub enum UserRole {
    Admin,
    User
}

impl UserRole {
    pub fn as_str(&self) -> &str {
        match self {
            UserRole::Admin => "admin",
            UserRole::User => "user",
        }
    }
}

#[derive(Debug, Serialize, Deserialize ,clone ,copy , PartialEq)]

pub struct User {
    id :uuid::Uuid,
    name :String,
    email :String,
    password :String,
    role :UserRole,
    created_at :DateTime<Utc>,
    updated_at :DateTime<Utc>,
}