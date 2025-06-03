use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct CreateWebsite {
    pub url: String,
    pub name: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct UpdateWebsite {
    pub url: Option<String>,
    pub name: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct CreateRegion {
    pub name: String,
}
