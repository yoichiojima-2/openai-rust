use crate::message::Message;
use reqwest::Client;
use serde_json;
use std::env;

const URL: &str = "https://api.openai.com/v1/chat/completions";
const MODEL: &str = "gpt-4o-mini";

pub async fn request(messages: &Vec<Message>) -> Result<serde_json::Value, reqwest::Error> {
    let client = Client::new();
    let api_key = env::var("OPENAI_API_KEY").unwrap();
    let body = build_body(messages);

    let res = client
        .post(URL)
        .header("Content-Type", "application/json")
        .header("Authorization", format!("Bearer {}", api_key))
        .json(&body)
        .send()
        .await
        .unwrap()
        .json()
        .await
        .unwrap();

    Ok(res)
}

fn build_body(messages: &Vec<Message>) -> serde_json::Value {
    serde_json::json!({
        "model": MODEL,
        "messages": messages,
    })
}

pub async fn get_first_choice(response: &serde_json::Value) -> Result<String, reqwest::Error> {
    let first_choice = response["choices"][0]["message"]["content"]
        .as_str()
        .unwrap();
    Ok(first_choice.to_string())
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::message::Role;

    fn build_messages() -> Vec<Message> {
        vec![Message {
            role: Role::User,
            content: "this is a test".to_string(),
        }]
    }

    #[test]
    fn test_build_body() {
        let messages = build_messages();

        let res = build_body(&messages);
        let expected = serde_json::json!({
            "model": MODEL,
            "messages": [
                {"role": "user", "content": "this is a test"}
            ]
        });

        assert_eq!(res, expected);
    }

    #[tokio::test]
    async fn test_request() {
        let messages = build_messages();
        let res = request(&messages).await.unwrap();

        let choices = &res["choices"];
        if let serde_json::Value::Array(choices) = choices {
            assert!(choices.len() > 0);
        }
    }

    #[tokio::test]
    async fn test_get_first_choice() {
        let messages = build_messages();
        let response = request(&messages).await.unwrap();
        let first_choice = get_first_choice(&response).await.unwrap();

        assert!(first_choice != "");
    }
}
