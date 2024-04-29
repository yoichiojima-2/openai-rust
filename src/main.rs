mod api_client;
mod error;

use serde_json::Value;
use std::env;

#[tokio::main]
async fn main() {
    match parse_args() {
        Ok(prompt) => match api_client::ask_gpt(prompt).await {
            Ok(response) => print_response(&response),
            Err(e) => eprintln!("Error: {}", e),
        },
        Err(_) => eprintln!("Usage: program <prompt>"),
    };
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
