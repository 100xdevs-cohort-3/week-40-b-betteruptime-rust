use serde::{Deserialize, Serialize};


#[derive(Serialize, Deserialize)]
pub struct CreateWebsite {
    pub url: String
}