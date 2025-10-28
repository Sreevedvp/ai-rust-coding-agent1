#!/bin/bash

set -e

echo "🚀 Setting up Personal Assistant Agent (Rust)"
echo "=============================================="
echo ""

# Colors
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
RED='\033[0;31m'
NC='\033[0m' # No Color

# Check if Rust is installed
if ! command -v cargo &> /dev/null; then
    echo -e "${RED}❌ Rust is not installed${NC}"
    echo "Install Rust from: https://rustup.rs/"
    echo "Run: curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh"
    exit 1
fi

echo -e "${GREEN}✅ Rust is installed${NC}"
rustc --version

# Check if Ollama is installed
if ! command -v ollama &> /dev/null; then
    echo -e "${RED}❌ Ollama is not installed${NC}"
    echo "Install Ollama: brew install ollama"
    exit 1
fi

echo -e "${GREEN}✅ Ollama is installed${NC}"

# Create project directory
PROJECT_DIR="assistant_agent"

if [ -d "$PROJECT_DIR" ]; then
    echo -e "${YELLOW}⚠️  Directory $PROJECT_DIR already exists${NC}"
    read -p "Do you want to continue? (y/n) " -n 1 -r
    echo
    if [[ ! $REPLY =~ ^[Yy]$ ]]; then
        exit 1
    fi
else
    echo "📁 Creating project directory..."
    mkdir -p "$PROJECT_DIR"
fi

cd "$PROJECT_DIR"

# Create directory structure
echo "📂 Creating directory structure..."
mkdir -p src/{config,llm,knowledge,memory,agent,cli,utils}
mkdir -p data/{knowledge_base,conversations,exports}
mkdir -p tests examples

# Create module files
echo "📝 Creating module files..."

# Create mod.rs files
touch src/lib.rs
for dir in config llm knowledge memory agent cli utils; do
    touch "src/$dir/mod.rs"
done

# Create main module files
touch src/config/settings.rs
touch src/llm/{ollama.rs,types.rs}
touch src/knowledge/{vectorstore.rs,embeddings.rs,documents.rs}
touch src/memory/{conversation.rs,storage.rs}
touch src/agent/{assistant.rs,chain.rs}
touch src/cli/{commands.rs,ui.rs}
touch src/utils/file.rs

# Create test files
touch tests/integration_tests.rs
touch examples/basic_usage.rs

# Create .env file
echo "⚙️  Creating .env file..."
cat > .env << 'EOF'
# Ollama Configuration
OLLAMA_HOST=http://localhost:11434
OLLAMA_MODEL=qwen2.5:7b
OLLAMA_EMBEDDING_MODEL=nomic-embed-text
OLLAMA_TEMPERATURE=0.7

# Storage
DATA_DIR=./data
KNOWLEDGE_BASE_DIR=./data/knowledge_base
CONVERSATIONS_DIR=./data/conversations

# Logging
RUST_LOG=info
EOF

echo -e "${GREEN}✅ .env file created${NC}"

# Create .gitignore
echo "📝 Creating .gitignore..."
cat > .gitignore << 'EOF'
# Rust
/target/
Cargo.lock
**/*.rs.bk

# Data
/data/
*.db

# Environment
.env.local

# IDE
.vscode/
.idea/
*.swp

# OS
.DS_Store
EOF

echo -e "${GREEN}✅ .gitignore created${NC}"

# Create README.md
echo "📝 Creating README.md..."
cat > README.md << 'EOF'
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
EOF

echo -e "${GREEN}✅ README.md created${NC}"

# Check if Cargo.toml exists, if not initialize cargo project
if [ ! -f "Cargo.toml" ]; then
    echo "📦 Initializing Cargo project..."
    cargo init --name assistant_agent
fi

echo ""
echo -e "${GREEN}✅ Project structure created successfully!${NC}"
echo ""
echo "📋 Next steps:"
echo "1. cd assistant_agent"
echo "2. Make sure Ollama is running: ollama serve"
echo "3. Pull required models:"
echo "   ollama pull qwen2.5:7b"
echo "   ollama pull nomic-embed-text"
echo "4. Copy the Rust implementation files provided"
echo "5. cargo build --release"
echo "6. cargo run"
echo ""
echo -e "${YELLOW}Note: I'll provide the complete Rust implementation next!${NC}"
