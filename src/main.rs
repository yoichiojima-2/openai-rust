mod api_client;
mod message;

use message::{Message, Role};

#[tokio::main]
async fn main() {
    let mut messages = vec![Message {
        role: Role::User,
        content: "this is a test".to_string(),
    }];
    let res = api_client::request(&messages).await.unwrap();
    let first_choice = api_client::get_first_choice(&messages).await.unwrap();
    messages.push(Message {
        role: Role::Assistant,
        content: first_choice,
    });
    println!("{:?}", messages);
}
