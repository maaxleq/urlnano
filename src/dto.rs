use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct SetUrlRequest {
    pub full_url: String,
}

#[derive(Serialize)]
pub struct SetUrlResponse {
    pub short_url: String,
}

#[derive(Serialize)]
pub struct GetUrlResponse {
    pub full_url: String,
}
