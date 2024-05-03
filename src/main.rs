mod api_client;
mod message;

use message::{Message, Role};
use std::io;

#[tokio::main]
async fn main() {
    interactive_chat().await;
}

async fn interactive_chat() {
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
