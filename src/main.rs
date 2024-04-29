use std::env;
use reqwest::{Client, Error};
use serde_json::{json, Value};


#[tokio::main]
async fn main() {
    let prompt = parse_args();
    match ask_gpt(prompt).await {
        Ok(response) => print_response(&response),
        Err(e) => eprintln!("Error: {}", e),
    }
}

async fn ask_gpt(prompt: String) -> Result<Value, Error> {
    let api_key = env::var("OPENAI_API_KEY").unwrap();
    let client = Client::new();
    let body = build_request_body(&prompt);

    let res = client.post("https://api.openai.com/v1/chat/completions")
        .header("Content-Type", "application/json")
        .header("Authorization", format!("Bearer {}", api_key))
        .json(&body)
        .send()
        .await?;

    res.json::<Value>().await

}

fn build_request_body(prompt: &str) -> Value {
    json!({
        "model": "gpt-3.5-turbo",
        "messages": [{"role": "user", "content": prompt}]
    })
}

fn parse_args() -> String {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        std::process::exit(1);
    }
    args[1].clone()
}


fn print_response(response: &Value) {
    if let Some(choice) = response["choices"].as_array() {
        if let Some(content) = choice[0]["message"]["content"].as_str() {
            println!("{}", content);
        }
    }
}
