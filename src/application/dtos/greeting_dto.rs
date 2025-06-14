use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct GreetingRequest {
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GreetingResponse {
    pub message: String,
}
