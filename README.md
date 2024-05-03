# OpenAI Rust Client

## Overview
This is an OpenAI API client implemented in Rust. It allows for interacting with the OpenAI API in a structured manner using commands from the command line. Currently, it supports interactive chatting and text translation.

## Requirements
- Rust
- Clap (for parsing command-line arguments)
- Tokio (for asynchronous operations)

## Installation
Clone the repository and build the project:

```bash
git clone https://github.com/yoichiojima-2/openai-rust.git
cd openai-rust
cargo build --release
```

## Usage
To use this client, run the compiled binary with the desired subcommand.

### Interactive Chat
Start an interactive chat session with the assistant:

```bash
./target/release/openai-rust interactive
```

This will allow you to send messages interactively and view responses from the OpenAI API.

### Translate
Translate text to Japanese (default language, configurable in the source code):

```bash
./target/release/openai-rust translate --text "Your text here"
```

## Configuration
You can modify the default language for translation by changing the LANG constant in the main Rust file.