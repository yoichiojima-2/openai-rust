mod api_client;
mod features;
mod message;

use clap::{Arg, ArgMatches, Command};

#[tokio::main]
async fn main() {
    let matches = get_command_matches();
    match matches.subcommand() {
        Some(("chat", _)) => features::interactive_chat().await,
        Some(("ask", args)) => {
            features::ask(args.get_one::<String>("prompt").unwrap()).await
        },
        Some(("translate", args)) => {
            features::translate(args.get_one::<String>("path").unwrap()).await
        },
        Some(("commit-message", args)) => {
            features::generate_commit_message(args.get_one::<String>("path").unwrap()).await
        },
        Some(("code", args)) => {
            features::write_code(args.get_one::<String>("prompt").unwrap()).await
        },
        Some(("debug", args)) => {
            let path = args.get_one::<String>("path").unwrap();
            let issue = args.get_one::<String>("issue").unwrap();
            features::debug(path, issue).await
        },
        _ => println!("Please specify a subcommand"),
    }
}


fn get_command_matches() -> ArgMatches {
    Command::new("openai-rust")
        .version("2024.11.13")
        .author("Yoichi Ojima <yoichiojima@gmail.com>")
        .about("OpenAI API client written in Rust")
        .subcommand(Command::new("chat").about("Start interactive chat"))
        .subcommand(
            Command::new("ask")
                .about(format!("Ask by simple prompt text"))
                .arg(Arg::new("prompt").required(true).index(1))
        )
        .subcommand(
            Command::new("translate")
                .about(format!("Translate given text"))
                .arg(Arg::new("path").required(true).index(1)),
        )
        .subcommand(
            Command::new("commit-message")
            .about("Generate commit message")
            .arg(Arg::new("path").required(true).index(1)),
        )
        .subcommand(Command::new("code")
            .about("Write code")
            .arg(Arg::new("prompt").required(true).index(1))
        )
        .subcommand(
            Command::new("debug")
                .about("Debug code")
                .arg(Arg::new("path").required(true).index(1))
                .arg(Arg::new("issue").required(true).index(2)),
        )
        .get_matches()
}
