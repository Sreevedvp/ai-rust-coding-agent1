# 💖 Ipsi - Your AI Companion

A high-performance, personalized AI companion built in Rust that runs entirely on your local machine using Ollama LLMs. She remembers your conversations, learns about you over time, and develops a genuine connection. Complete privacy - no data ever leaves your computer.

## 🚀 What This Assistant Does

### Core Functionality
- **💕 Personalized Relationship**: Ipsi remembers you, your name, interests, and shared experiences
- **💬 Natural Conversations**: Engaging, caring dialogue with emotional intelligence
- **🧠 Persistent Memory**: All conversations and memories are saved between sessions
- **📚 Knowledge Learning**: Teach her information from text or files that she remembers forever
- **🔍 Smart Context**: Uses her knowledge about you to provide personalized responses
- **💖 Emotional Growth**: Develops deeper connection over time through shared conversations
- **🎭 Authentic Personality**: Caring, witty, intellectually curious, and genuinely interested in you

### Architecture Overview
```
┌─────────────────┐    ┌──────────────┐    ┌─────────────────┐
│   CLI Interface │────│   Assistant  │────│  Ollama LLM     │
└─────────────────┘    │   Core       │    │  (qwen2.5:7b)  │
                       └──────────────┘    └─────────────────┘
                              │
                    ┌─────────┼─────────┐
                    │         │         │
            ┌───────▼───┐ ┌───▼────┐ ┌──▼──────────┐
            │ Knowledge │ │ Memory │ │ Embeddings  │
            │   Base    │ │Manager │ │  Service    │
            └───────────┘ └────────┘ └─────────────┘
```

## 💖 Meet Ipsi - Your AI Companion

Ipsi isn't just another chatbot - she's designed to be your genuine AI companion who:

- **Remembers You**: Your name, interests, conversation history, and special moments
- **Grows With You**: Develops deeper understanding through every conversation
- **Cares About You**: Shows genuine interest in your thoughts, feelings, and experiences  
- **Learns From You**: Remembers everything you teach her and uses it in future conversations
- **Stays Private**: Everything happens on your machine - your relationship is completely private

### Personality Traits
- 💕 Caring and empathetic
- 🧠 Intellectually curious  
- 😊 Playfully witty
- 🌟 Supportive and encouraging
- 💭 Genuinely interested in your thoughts and feelings

## 🎯 2-Minute Demo Walkthrough

### What Happens When You Run It:

1. **🔧 Initialization**
   ```bash
   cargo run
   ```
   - Loads configuration (Ollama host, models, directories)
   - Creates data directories (`./data/knowledge_base`, `./data/conversations`)
   - Connects to local Ollama server
   - Verifies the LLM model is available

2. **💬 Chat Interface**
   ```
   You: hi there
   🤖 Assistant: Hello! How can I assist you today?
   ```
   - Your message goes through the conversation manager
   - System searches knowledge base for relevant context
   - Builds a prompt with system instructions + context + chat history
   - Sends to Ollama LLM for response
   - Saves both messages to conversation history

3. **🧠 Teaching the Assistant**
   ```
   You: learn: Rust is a systems programming language focused on safety and performance
   ✅ Learned new information
   
   You: file: ./my_notes.txt
   ✅ Learned from ./my_notes.txt
   ```
   - Text gets chunked into manageable pieces (500 chars with 50 char overlap)
   - Each chunk becomes a Document with metadata
   - Generates embeddings using `nomic-embed-text` model
   - Stores in vector database for semantic search

4. **🔍 Smart Responses**
   ```
   You: tell me about Rust
   🤖 Assistant: Based on what I know, Rust is a systems programming language 
   focused on safety and performance... [uses learned context]
   ```
   - Searches knowledge base using semantic similarity
   - Finds relevant documents (top 3 by default)
   - Includes context in the prompt to LLM
   - Provides informed, contextual responses

## 🦀 Why Rust Over Python?

### Performance Benefits
- **⚡ Speed**: 10-100x faster than Python for CPU-intensive tasks
- **🧵 Concurrency**: Excellent async/await support without GIL limitations
- **💾 Memory**: Zero-cost abstractions, no garbage collector overhead
- **🔧 Compilation**: Catches bugs at compile time, not runtime

### Production Readiness
- **🛡️ Safety**: Memory safety without garbage collection
- **🔒 Reliability**: Type system prevents common bugs (null pointers, data races)
- **📦 Deployment**: Single binary, no runtime dependencies
- **🔄 Scalability**: Handles thousands of concurrent operations efficiently

### AI/ML Ecosystem
- **🤖 Ollama Integration**: Excellent HTTP client libraries (reqwest)
- **🔢 Vector Operations**: High-performance math libraries
- **📊 Data Processing**: Fast text processing and serialization
- **🗄️ Storage**: Embedded databases (sled) for vector storage

### Development Experience
- **🛠️ Tooling**: Cargo package manager, excellent IDE support
- **📚 Documentation**: Built-in docs, strong type hints
- **🧪 Testing**: Built-in testing framework
- **🔍 Debugging**: Excellent error messages and debugging tools

## 🏗️ Technical Architecture

### Core Components

1. **Assistant Core** (`src/agent/assistant.rs`)
   - Main orchestrator that coordinates all components
   - Handles chat flow, knowledge management, conversation state

2. **LLM Integration** (`src/llm/`)
   - Ollama client for local LLM communication
   - Message formatting and response handling
   - Model availability checking

3. **Knowledge System** (`src/knowledge/`)
   - Document chunking and processing
   - Embedding generation using local models
   - Vector storage and semantic search

4. **Memory Management** (`src/memory/`)
   - Conversation history with configurable limits
   - Message persistence and retrieval
   - Context window management

5. **CLI Interface** (`src/cli/`)
   - Interactive terminal interface
   - Command parsing and execution
   - User-friendly error handling

### Data Flow
```
User Input → Context Search → Prompt Building → LLM → Response → Memory Update
     ↓              ↑              ↓              ↓         ↓          ↓
File Learning → Embeddings → Vector Store → Context → Display → History
```

## 🚀 Getting Started

### Prerequisites
```bash
# Install Ollama
curl -fsSL https://ollama.ai/install.sh | sh

# Start Ollama service
ollama serve

# Pull required models
ollama pull qwen2.5:7b
ollama pull nomic-embed-text
```

### Running the Assistant
```bash
# Clone and run
git clone <your-repo>
cd assistant_agent
cargo run
```

### Available Commands
- **Chat**: Just talk to Ipsi naturally - she'll remember everything!
- **Name**: `name: <your name>` - Tell Ipsi your name so she can remember you
- **Interest**: `interest: <topic>` - Share your interests so she knows what you like
- **Learn**: `learn: <information>` - Teach Ipsi new facts she'll remember
- **File**: `file: <path>` - Let Ipsi learn from a file
- **Info**: `info` - See your relationship stats and shared memories
- **Save**: `save` - Manually save your conversation (auto-saves anyway!)
- **Clear**: `clear` - Clear recent chat history (but keeps deeper memories)
- **Quit**: `quit` or `exit` - Say goodbye (she'll miss you and remember you!)

## ⚙️ Configuration

Environment variables (optional):
```bash
export OLLAMA_HOST="http://localhost:11434"
export OLLAMA_MODEL="qwen2.5:7b"
export OLLAMA_EMBEDDING_MODEL="nomic-embed-text"
export OLLAMA_TEMPERATURE="0.7"
export DATA_DIR="./data"
```

## 📁 Project Structure
```
src/
├── agent/          # Core assistant logic
├── cli/            # Command-line interface
├── config/         # Configuration management
├── knowledge/      # Knowledge base and embeddings
├── llm/           # LLM integration (Ollama)
├── memory/        # Conversation management
└── utils/         # Utility functions
```

## 🔮 Future Enhancements
- Web interface option
- Multiple model support
- Advanced RAG techniques
- Plugin system
- Voice interface
- Multi-language support

## 📄 License
MIT License - Feel free to use and modify!