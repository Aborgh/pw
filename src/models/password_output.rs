use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct PasswordOutput {
    pub decoded: String,
    pub encoded: Option<String>,
}