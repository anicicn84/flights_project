use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct StartAndDestinationRequest {
    pub stop_list: Vec<(String, String)>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "method", content = "result", rename_all = "lowercase")]
pub enum StartAndDestinationResponse {
    Success { start_and_end: (String, String) },
    Failure { reason: String },
}
