use crate::error::MyError;
use reqwest::Client;
use serde_json::{json, Value};
use std::env;

const API_URL: &str = "https://api.openai.com/v1/chat/completions";

pub async fn ask_gpt(prompt: String) -> Result<Value, MyError> {
    let api_key = env::var("OPENAI_API_KEY").map_err(|_| MyError::ApiKeyNotSet)?;

    let client = build_client();
    let body = build_request_body(&prompt);

    let res = client
        .post(API_URL)
        .header("Content-Type", "application/json")
        .header("Authorization", format!("Bearer {}", api_key))
        .json(&body)
        .send()
        .await?;

    Ok(res.json::<Value>().await?)
}

pub fn build_client() -> Client {
    Client::new()
}

fn build_request_body(prompt: &str) -> Value {
    json!({
        "model": "gpt-3.5-turbo",
        "messages": [{"role": "user", "content": prompt}]
    })
}
