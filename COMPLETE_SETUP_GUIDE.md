# Complete Setup Guide - Personal Assistant Agent (Rust)

## 📋 Prerequisites

1. **Rust** - Install from https://rustup.rs/
   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   source $HOME/.cargo/env
   ```

2. **Ollama** - Install on Mac
   ```bash
   brew install ollama
   ```

## 🚀 Quick Setup (5 Minutes)

### Step 1: Create Project Structure

```bash
# Navigate to where you want the project
cd ~/Desktop/Personal  # Or your preferred location

# Create the directory structure
mkdir -p assistant_agent/src/{config,llm,knowledge,memory,agent,cli,utils}
mkdir -p assistant_agent/data/{knowledge_base,conversations,exports}

cd assistant_agent
```

### Step 2: Create All Files

Run these commands to create all necessary files:

```bash
# Create main module files
touch src/main.rs
touch src/lib.rs

# Create config module
touch src/config/mod.rs
touch src/config/settings.rs

# Create llm module
touch src/llm/mod.rs
touch src/llm/ollama.rs
touch src/llm/types.rs

# Create knowledge module
touch src/knowledge/mod.rs
touch src/knowledge/vectorstore.rs
touch src/knowledge/embeddings.rs
touch src/knowledge/documents.rs

# Create memory module
touch src/memory/mod.rs
touch src/memory/conversation.rs
touch src/memory/storage.rs

# Create agent module
touch src/agent/mod.rs
touch src/agent/assistant.rs
touch src/agent/chain.rs

# Create cli module
touch src/cli/mod.rs
touch src/cli/commands.rs
touch src/cli/ui.rs

# Create utils module
touch src/utils/mod.rs
touch src/utils/file.rs

# Create config files
touch Cargo.toml
touch .env
touch .gitignore
touch README.md
```

### Step 3: Copy Code Files

Now copy the code from the artifacts into each file:

1. **Cargo.toml** - Copy from "Cargo.toml - Dependencies" artifact
2. **src/main.rs** - Copy from "Complete src/main.rs" artifact
3. **src/lib.rs** - Copy from "Complete src/lib.rs" artifact
4. **All other modules** - Copy from "All Module Files" and "Remaining Module Files" artifacts

### Step 4: Create .env File

```bash
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
```

### Step 5: Create .gitignore

```bash
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
```

### Step 6: Start Ollama (IMPORTANT!)

Open a **new terminal window** and run:

```bash
ollama serve
```

**Keep this terminal running!**

### Step 7: Download Models

In **another terminal**, download the required models:

```bash
ollama pull qwen2.5:7b
ollama pull nomic-embed-text
```

This will take a few minutes depending on your internet speed.

### Step 8: Build the Project

Back in your project directory:

```bash
cd ~/Desktop/Personal/assistant_agent

# Build the project (first time will take a while)
cargo build --release
```

### Step 9: Run Your Assistant! 🎉

```bash
cargo run --release
```

Or use the compiled binary:

```bash
./target/release/assistant
```

## 📖 Usage Examples

### Interactive Chat (Default)
```bash
cargo run --release
# or
cargo run --release -- chat
```

Then type your messages:
```
You: What's the weather like?
🤖 Assistant: I don't have real-time weather data, but I can help you with other questions!

You: learn: I live in San Francisco and love hiking on weekends
✅ Learned new information

You: Where do I live?
🤖 Assistant: You live in San Francisco and enjoy hiking on weekends!
```

### Learn from Text
```bash
cargo run --release -- learn --text "My favorite programming language is Rust because it's fast and safe"
```

### Learn from File
```bash
# Create a test file
echo "I work as a software engineer at TechCorp. I specialize in backend development." > notes.txt

# Learn from it
cargo run --release -- learn --file notes.txt
```

### Export Knowledge
```bash
cargo run --release -- export --output my_knowledge.json
```

### Show System Info
```bash
cargo run --release -- info
```

### Clear History
```bash
cargo run --release -- clear
```

### Save Conversation
```bash
cargo run --release -- save
# or with custom name
cargo run --release -- save --name "important_chat"
```

## 🔧 Troubleshooting

### "Cannot connect to Ollama"
1. Make sure Ollama is running:
   ```bash
   ollama serve
   ```
2. Check it's accessible:
   ```bash
   curl http://localhost:11434
   ```

### "Model not found"
Download the required models:
```bash
ollama pull qwen2.5:7b
ollama pull nomic-embed-text
```

### "Compilation errors"
Make sure you have the latest Rust:
```bash
rustup update
```

### Slow performance
The first run compiles everything. Subsequent runs will be fast!

### Port already in use
If Ollama's port is taken, change it in `.env`:
```env
OLLAMA_HOST=http://localhost:11435  # Different port
```

## 🎯 File Mapping Reference

Here's where each piece of code goes:

| Artifact Section | File Location |
|-----------------|---------------|
| Cargo.toml | `Cargo.toml` |
| src/lib.rs | `src/lib.rs` |
| src/main.rs | `src/main.rs` |
| config/mod.rs | `src/config/mod.rs` |
| config/settings.rs | `src/config/settings.rs` |
| llm/mod.rs | `src/llm/mod.rs` |
| llm/types.rs | `src/llm/types.rs` |
| llm/ollama.rs | `src/llm/ollama.rs` |
| knowledge/* | `src/knowledge/*.rs` |
| memory/* | `src/memory/*.rs` |
| agent/* | `src/agent/*.rs` |
| cli/* | `src/cli/*.rs` |
| utils/* | `src/utils/*.rs` |

## 📁 Final Project Structure

```
assistant_agent/
├── Cargo.toml
├── .env
├── .gitignore
├── README.md
├── src/
│   ├── main.rs
│   ├── lib.rs
│   ├── config/
│   │   ├── mod.rs
│   │   └── settings.rs
│   ├── llm/
│   │   ├── mod.rs
│   │   ├── ollama.rs
│   │   └── types.rs
│   ├── knowledge/
│   │   ├── mod.rs
│   │   ├── vectorstore.rs
│   │   ├── embeddings.rs
│   │   └── documents.rs
│   ├── memory/
│   │   ├── mod.rs
│   │   ├── conversation.rs
│   │   └── storage.rs
│   ├── agent/
│   │   ├── mod.rs
│   │   ├── assistant.rs
│   │   └── chain.rs
│   ├── cli/
│   │   ├── mod.rs
│   │   ├── commands.rs
│   │   └── ui.rs
│   └── utils/
│       ├── mod.rs
│       └── file.rs
└── data/
    ├── knowledge_base/
    ├── conversations/
    └── exports/
```

## ✅ Verification Checklist

- [ ] Rust installed (`rustc --version`)
- [ ] Ollama installed (`ollama --version`)
- [ ] Ollama is running (`ollama serve` in separate terminal)
- [ ] Models downloaded (`ollama list` shows qwen2.5:7b and nomic-embed-text)
- [ ] All files created with correct content
- [ ] `.env` file configured
- [ ] Project builds successfully (`cargo build --release`)
- [ ] Assistant runs (`cargo run --release`)

## 🚀 Next Steps

Once everything is working:

1. **Teach it about yourself**: Use `learn:` commands to add personal information
2. **Try different queries**: Ask questions to test knowledge retrieval
3. **Export your knowledge**: Save your knowledge base for backup
4. **Customize settings**: Edit `.env` to adjust temperature, chunk sizes, etc.
5. **Scale it up**: Add more features based on your needs!

## 💡 Tips

- Use `cargo run` for development (faster compilation)
- Use `cargo run --release` for production (faster execution)
- Keep conversations by using the `save` command
- Regularly export your knowledge base as backup
- The assistant gets smarter as you teach it more!

Happy coding! 🎉
