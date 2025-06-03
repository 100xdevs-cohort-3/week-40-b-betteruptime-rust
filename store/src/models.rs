use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct Website {
    pub id: String,
    pub url: String,
    pub name: Option<String>,
    pub time_added: NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "WebsiteStatus", rename_all = "PascalCase")]
pub enum WebsiteStatus {
    Up,
    Down,
    Unknown,
}

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct WebsiteTick {
    pub id: String,
    pub response_time_ms: i32,
    pub status: WebsiteStatus,
    pub region_id: String,
    pub website_id: String,
}

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct Region {
    pub id: String,
    pub name: String,
}
