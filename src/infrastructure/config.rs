use serde::{Deserialize, Serialize};

#[allow(dead_code)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppConfig {
    pub server_port: u16,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            server_port: 8000,
        }
    }
}
