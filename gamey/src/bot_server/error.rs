use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct ErrorResponse {
    api_version: Option<String>,
    bot_id: Option<String>,
    message: String,
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
