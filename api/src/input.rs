use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct CreateWebsite {
    pub url: String,
    pub name: Option<String>,
    pub email: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct Website {
    pub url: String,
    pub name: Option<String>,
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

// Simplified response structure for frontend - only time and value
#[derive(Serialize, Deserialize, Debug)]
pub struct TimeSeriesPoint {
    pub time: String,
    pub value: f64,
}

// Configuration struct for InfluxDB
#[derive(Clone)]
pub struct InfluxConfig {
    pub url: String,
    pub token: String,
    pub org: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LastDowntime {
    pub time: String,
    pub value: f64,
}
