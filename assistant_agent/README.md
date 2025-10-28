# Personal Assistant Agent

A scalable, privacy-focused personal assistant built with Rust and local LLMs.

## Features

- 🔒 100% Private - All data stays on your machine
- 🚀 Fast - Built with Rust for maximum performance
- 🧠 Smart - Uses Qwen2.5:7b for intelligent responses
- 📚 Learning - Continuously learns from your inputs
- 💾 Persistent - Saves knowledge and conversations

## Prerequisites

- Rust (1.75+)
- Ollama
- Qwen2.5:7b model
- nomic-embed-text model

## Installation

```bash
# Install Ollama models
ollama pull qwen2.5:7b
ollama pull nomic-embed-text

# Build the project
cargo build --release

# Run the assistant
cargo run --release
```

## Usage

### Interactive Chat
```bash
cargo run -- chat
# or simply
cargo run
```

### Learn from Text
```bash
cargo run -- learn --text "Your information here"
```

### Learn from File
```bash
cargo run -- learn --file /path/to/file.txt
```

### Export Knowledge
```bash
cargo run -- export --output my_knowledge.json
```

### Clear History
```bash
cargo run -- clear
```

### System Info
```bash
cargo run -- info
```

## Development

```bash
# Run tests
cargo test

# Run with verbose logging
cargo run -- --verbose

# Build optimized release
cargo build --release
```

## Architecture

- `agent/` - Core assistant logic
- `llm/` - Ollama client and LLM interactions
- `knowledge/` - Vector store and embeddings
- `memory/` - Conversation history management
- `cli/` - Command-line interface
- `config/` - Configuration management
- `utils/` - Utility functions

## License

MIT
