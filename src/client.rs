use serde::{Deserialize, Serialize};

#[derive(Default, Serialize)]
pub(crate) struct InnerRequest {
    pub(crate) token: String,
    pub(crate) user: String,
    pub(crate) device: Option<String>,
    pub(crate) title: Option<String>,
    pub(crate) message: String,
    pub(crate) html: Option<bool>,
    pub(crate) timestamp: Option<u64>,
    pub(crate) priority: Option<u8>,
    pub(crate) url: Option<String>,
    pub(crate) url_title: Option<String>,
    pub(crate) sound: Option<String>,
}

#[derive(Deserialize)]
pub(crate) struct InnerResponse {
    pub(crate) status: u64,
    pub(crate) request: String,
    pub(crate) errors: Option<Vec<String>>,
}
