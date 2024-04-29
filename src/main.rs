use std::env;
use reqwest::Client;
use serde_json::{json, Value};
use thiserror::Error;


#[derive(Debug, Error)]
enum MyError {
    #[error("API key not set in environment")]
    ApiKeyNotSet,
    #[error("Request failed: {0}")]
    ReqwestError(#[from] reqwest::Error),
}


const API_URL: &str = "https://api.openai.com/v1/chat/completions";

#[tokio::main]
async fn main() {
    match parse_args() {
        Ok(prompt) => {
            match ask_gpt(prompt).await {
                Ok(response) => print_response(&response),
                Err(e) => eprintln!("Error: {}", e),
            }
        }
        Err(_) => eprintln!("Usage: program <prompt>"),
    };
}


async fn ask_gpt(prompt: String) -> Result<Value, MyError> {
    let api_key = env::var("OPENAI_API_KEY").map_err(|_| MyError::ApiKeyNotSet)?;

    let client = build_client();
    let body = build_request_body(&prompt);

    let res = client.post(API_URL)
        .header("Content-Type", "application/json")
        .header("Authorization", format!("Bearer {}", api_key))
        .json(&body)
        .send()
        .await?;

    Ok(res.json::<Value>().await?)
}


fn build_client() -> Client {
    Client::new()
}


fn build_request_body(prompt: &str) -> Value {
    json!({
        "model": "gpt-3.5-turbo",
        "messages": [{"role": "user", "content": prompt}]
    })
}


fn parse_args() -> Result<String, ()> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        Err(())
    } else {
        Ok(args[1].clone())
    }
}


fn print_response(response: &Value) {
    if let Some(choice) = response["choices"].as_array() {
        if let Some(content) = choice[0]["message"]["content"].as_str() {
            println!("{}", content);
        }
    }
}