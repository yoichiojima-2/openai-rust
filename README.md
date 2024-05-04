# openai-rust

The `openai-rust` project is a command-line interface (CLI) tool written in Rust, designed to interact with OpenAI APIs. It supports multiple features like interactive chatting, translation, and generating commit messages.

## Features

- **Interactive Chat**: Engage in an interactive chat session.
- **Translate**: Translate text from a specified file.
- **Generate Commit Message**: Automatically generate a commit message for a given change.

## Installation

Clone the repository and build the project:

```bash
git clone https://github.com/your-github/openai-rust.git
cd openai-rust
cargo build --release
```

## Usage

Run the tool using the following commands:

### Start Interactive Chat

```bash
Copy code
cargo run -- interactive
```

### Translate Text
```bash
Copy code
cargo run -- translate <path-to-text-file>
```

### Generate Commit Message
```bash
Copy code
cargo run -- commit-message <path-to-file>
```

## Dependencies

- Rust
- Clap
- Tokio
