use serde::{Deserialize, Serialize};

#[allow(dead_code)]
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Role {
    User,
    System,
    Assistant,
}

#[allow(dead_code)]
#[derive(Debug, Serialize, Deserialize)]
pub struct Message {
    pub role: Role,
    pub content: String,
}
