use std::fs::File;
use std::io::{self, Read};
use std::process::Command;
use std::str;

use crate::api_client;
use crate::message::{Message, Role};

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

pub async fn generate_commit_message(path: &str) {
    let diff = git_diff(path);
    let messages: Vec<Message> = vec![
        Message {
            role: Role::System,
            content: "Make a commit message from the diff".to_string(),
        },
        Message {
            role: Role::User,
            content: diff,
        },
    ];

    let response = api_client::request(&messages).await.unwrap();
    let first_choice = api_client::get_first_choice(&response).await.unwrap();

    println!("{}", first_choice);
}

fn git_diff(path: &str) -> String {
    let output = Command::new("git")
        .arg("diff")
        .arg(path)
        .output()
        .unwrap();

    str::from_utf8(&output.stdout).unwrap().to_string()
}


pub async fn ask(prompt: &str) {
    let messages: Vec<Message> = vec![
        Message {
            role: Role::User,
            content: prompt.to_string(),
        },
    ];

    let response = api_client::request(&messages).await.unwrap();
    let first_choice = api_client::get_first_choice(&response).await.unwrap();

    println!("{}", first_choice);
}

pub async fn write_code(prompt: &str) {
    let messages: Vec<Message> = vec![
        Message {
            role: Role::System,
            content: "Write a code that does what's asked".to_string(),
        },
        Message {
            role: Role::User,
            content: prompt.to_string(),
        },
    ];

    let response = api_client::request(&messages).await.unwrap();
    let first_choice = api_client::get_first_choice(&response).await.unwrap();

    println!("{}", first_choice);
}

pub async fn debug(path: &str, issue: &str) {
    let mut code = String::new();
    File::open(path).unwrap().read_to_string(&mut code).unwrap();

    let messages: Vec<Message> = vec![
        Message {
            role: Role::System,
            content: format!("Debug given code. code and issue will be given."),
        },
        Message {
            role: Role::User,
            content: code,
        },
        Message {
            role: Role::User,
            content: issue.to_string(),
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
