use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct ErrorResponse {
    pub api_version: Option<String>,
    pub bot_id: Option<String>,
    pub message: String,
}

impl ErrorResponse {
    pub fn error(message: &str, api_version: Option<String>, bot_id: Option<String>) -> Self {
        Self {
            bot_id,
            api_version,
            message: message.to_string(),
        }
    }
}
