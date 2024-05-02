mod api_client;
mod message;

use message::{Message, Role};
use std::io;

#[tokio::main]
async fn main() {
    let mut messages: Vec<Message> = Vec::new();

    loop {
        let mut input = String::new();

        io::stdin().read_line(&mut input).unwrap();

        messages.push(Message {
            role: Role::User,
            content: input,
        });

        let first_choice = api_client::get_first_choice(&messages).await.unwrap();

        println!("{}", first_choice);

        messages.push(Message {
            role: Role::Assistant,
            content: first_choice,
        });

        println!("");
    }
}
