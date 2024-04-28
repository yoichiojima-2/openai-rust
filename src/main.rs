use std::env;
use reqwest::{Client, Error};
use serde_json::{json, Value};


#[tokio::main]
async fn main() {
    let res = fetch_data().await;
    println!("{:?}", res);
}

async fn fetch_data() -> Result<Value, Error> {
    let api_key = env::var("OPENAI_APIKEY").unwrap();
    let client = Client::new();
    let body = build_request_body("Say this is a test");

    let res = client.post("https://api.openai.com/v1/chat/completions")
        .header("Content-Type", "application/json")
        .header("Authorization", format!("Bearer {}", api_key))
        .json(&body)
        .send()
        .await?;

    let json_response = res.json::<Value>().await?;

    Ok(json_response)
}

fn build_request_body(prompt: &str) -> Value {
    json!({
        "model": "gpt-3.5-turbo",
        "messages": [{"role": "user", "content": prompt}]
    })
}
