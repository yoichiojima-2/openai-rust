use crate::api_client;
use crate::message::{Message, Role};
use std::io::{self, Read};
use std::fs::File;

const LANG: &str = "Japanese";

pub async fn interactive_chat() {
    println!("Type your message and press Enter to send it to the assistant.\n");

    let mut messages: Vec<Message> = Vec::new();

    loop {
        let mut input = String::new();

        io::stdin().read_line(&mut input).unwrap();

        messages.push(Message {
            role: Role::User,
            content: input,
        });

        let response = api_client::request(&messages).await.unwrap();
        let first_choice = api_client::get_first_choice(&response).await.unwrap();

        println!("{}", first_choice);

        messages.push(Message {
            role: Role::Assistant,
            content: first_choice,
        });

        println!("");
    }
}

pub async fn translate(path: &str) {
    let mut text = String::new();
    File::open(path).unwrap().read_to_string(&mut text).unwrap();

    let messages: Vec<Message> = vec![
        Message {
            role: Role::System,
            content: format!("Translate given text to {}", LANG),
        },
        Message {
            role: Role::User,
            content: text,
        },
    ];

    let response = api_client::request(&messages).await.unwrap();
    let first_choice = api_client::get_first_choice(&response).await.unwrap();

    println!("{}", first_choice);
}

#[cfg(test)]
mod test {
    use super::*;

    #[tokio::test]
    async fn test_translate() {
        let path = "data/test.txt";
        translate(path).await;
    }
}
