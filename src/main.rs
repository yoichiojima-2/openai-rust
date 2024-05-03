mod api_client;
mod features;
mod message;

use clap::{Arg, ArgMatches, Command};

#[tokio::main]
async fn main() {
    let matches = get_command_matches();
    match matches.subcommand() {
        Some(("interactive", _)) => features::interactive_chat().await,
        Some(("translate", args)) => {
            features::translate(args.get_one::<String>("text").unwrap()).await
        }
        _ => println!("Please specify a subcommand"),
    }
}

fn get_command_matches() -> ArgMatches {
    Command::new("openai-rust")
        .version("2024.5.3")
        .author("Yoichi Ojima <yoichiojima@gmail.com>")
        .about("OpenAI API client written in Rust")
        .subcommand(Command::new("interactive").about("Start interactive chat"))
        .subcommand(
            Command::new("translate")
                .about(format!("Translate given text"))
                .arg(
                    Arg::new("text")
                        .help("Text to translate")
                        .required(true)
                        .value_parser(clap::value_parser!(String)),
                ),
        )
        .get_matches()
}
