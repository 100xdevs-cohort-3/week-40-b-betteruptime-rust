use influxdb2::FromDataPoint;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct CreateWebsite {
    pub url: String,
    pub name: Option<String>,
    pub email: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct UpdateWebsite {
    pub url: Option<String>,
    pub name: Option<String>,
    pub email: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct CreateRegion {
    pub name: String,
}

#[derive(Serialize, Deserialize)]
pub struct TickQuery {
    pub days: Option<i64>,
    pub region: Option<String>,
}

#[derive(Serialize, Deserialize, FromDataPoint, Default, Debug)]
pub struct WebsiteStatus {
    pub id: String,
    pub response_time_ms: f64,
    pub status: String,
    pub region_id: String,
    pub website_id: String,
    pub timestamp: i64,
}
