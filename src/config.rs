use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Deserialize, Serialize)]
pub struct Config {
    pub update_interval_ms: u64,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            update_interval_ms: 1000,
        }
    }
}

impl Config {
    pub const VERSION: u64 = 1;
}
