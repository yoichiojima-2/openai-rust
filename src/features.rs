use crate::api_client;
use crate::message::{Message, Role};
use std::io;

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

pub async fn translate(text: &str) {
    let messages: Vec<Message> = vec![
        Message {
            role: Role::System,
            content: format!("Translate given text to {}", LANG),
        },
        Message {
            role: Role::User,
            content: text.to_string(),
        },
    ];

    let response = api_client::request(&messages).await.unwrap();
    let first_choice = api_client::get_first_choice(&response).await.unwrap();

    println!("{}", first_choice);
}
