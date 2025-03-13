use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

#[derive(Deserialize)]
pub struct UserPagination {
    pub size: u16,
    pub offset: u16
}

#[derive(Deserialize)]
pub struct UserPayload {
    pub username: String,
    pub email: String,
    pub password: String
}

#[derive(Serialize)]
pub struct UserInfo {
    pub id: String,
    pub username: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    pub avatar_url: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>
}

#[derive(Serialize)]
pub struct TokenResponse {
    pub token: String
}

pub struct TokenClaims {
    pub sub: String,
    pub iat: DateTime<Utc>,
    pub exp: DateTime<Utc>
}



#[derive(Deserialize)]
pub struct FileQuery {
    pub id: String
}

#[derive(Serialize)]
pub struct FileInfo {
    pub id: String,
    pub owner: String,
    pub mime: String,
    pub size: u64,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>
}