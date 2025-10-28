# 🤖 Personal Assistant Agent

A high-performance, scalable personal assistant built in Rust that runs entirely on your local machine using Ollama LLMs. No data leaves your computer - complete privacy and control.

## 🚀 What This Assistant Does

### Core Functionality
- **💬 Interactive Chat**: Natural conversations with local LLMs (qwen2.5:7b by default)
- **🧠 Knowledge Learning**: Teach it information from text or files that it remembers
- **🔍 Smart Context**: Searches its knowledge base to provide relevant, contextual responses
- **💾 Conversation Memory**: Maintains chat history with configurable limits
- **📁 File Learning**: Can learn from any text file to expand its knowledge
- **💾 Conversation Saving**: Save and manage your chat sessions

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
- **Chat**: Just type your message
- **Learn**: `learn: <information>` - Teach new facts
- **File**: `file: <path>` - Learn from a file
- **Save**: `save` - Save current conversation
- **Clear**: `clear` - Clear chat history
- **Quit**: `quit` or `exit` - Exit the program

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