mod api_client;
mod message;

use std::io;
use clap::{Command, Arg};
use message::{Message, Role};

#[tokio::main]
async fn main() {
    let matches = Command::new("openai-rust")
        .version("2024.5.3")
        .author("Yoichi Ojima <yoichiojima@gmail.com>")
        .about("OpenAI API client written in Rust")
        .subcommand(Command::new("interactive")
            .about("Start interactive chat")
        )
        .subcommand(Command::new("translate")
            .about("Translate given text to Japanese")
            .arg(Arg::new("text")
                .help("Text to translate")
                .required(true)
                .value_parser(clap::value_parser!(String))
            )
        )
        .get_matches();
    
    match matches.subcommand() {
        Some(("interactive", _)) => interactive_chat().await,
        Some(("translate", args)) => translate(args.get_one::<String>("text").unwrap()).await,
        _ => println!("Please specify a subcommand"),
    
    }
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

async fn translate(text: &str) {
    let messages: Vec<Message> = vec![
        Message {
            role: Role::System,
            content: "Translate given text to Japanese".to_string(),
        },
        Message {
            role: Role::User,
            content: text.to_string(),
        }
    ];

    let response = api_client::request(&messages).await.unwrap();
    let first_choice = api_client::get_first_choice(&response).await.unwrap();

    println!("{}", first_choice);
}
