# openai-rust
The openai-rust project is a command-line interface (CLI) tool written in Rust, designed to interact with OpenAI APIs. It supports multiple features like interactive chatting, translation, generating commit messages, writing code, and debugging code.

## Features
- Interactive Chat: Engage in an interactive chat session.
- Translate: Translate text from a specified file.
- Generate Commit Message: Automatically generate a commit message for a given change.
- Write Code: Generate code based on a given prompt.
- Debug Code: Debug code with a specified issue.
- Installation
- Clone the repository and build the project:

```bash
git clone https://github.com/yoichiojima-2/openai-rust.git
cd openai-rust
cargo build --release
```

## Usage

Run the tool using the following commands:

Start Interactive Chat

```bash
cargo run -- interactive
```

### Translate Text

```bash
cargo run -- translate <path-to-text-file>
```

### Generate Commit Message

```bash
cargo run -- commit-message <path-to-file>
```

### Write Code

```bash
cargo run -- code <prompt>
```

### Debug Code

```bash
cargo run -- debug <path-to-file> <issue>
```

## Dependencies
- Rust
- Clap
- Tokio

## Example Commands
### Start Interactive Chat
To start an interactive chat session:

```bash
cargo run -- interactive
```

### Translate Text
To translate text from a specified file:

```bash
cargo run -- translate path/to/text/file.txt
```

## Generate Commit Message
To generate a commit message for a given file:

```bash
cargo run -- commit-message path/to/file.txt
```

## Write Code
To generate code based on a given prompt:

```bash
cargo run -- code "Implement a binary search algorithm"
```

## Debug Code
To debug code with a specified issue:

```bash
cargo run -- debug path/to/code/file.rs "The function does not return the correct value"
```

Ensure that you have the necessary permissions and access to the specified files when running these commands.

# License
This project is licensed under the MIT License. See the LICENSE file for more details.

