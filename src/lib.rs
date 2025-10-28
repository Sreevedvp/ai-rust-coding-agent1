// src/lib.rs
pub mod agent;
pub mod cli;
pub mod config;
pub mod knowledge;
pub mod llm;
pub mod memory;
pub mod utils;

use thiserror::Error;

#[derive(Error, Debug)]
pub enum AssistantError {
    #[error("Ollama API error: {0}")]
    OllamaError(String),
    
    #[error("Knowledge base error: {0}")]
    KnowledgeError(String),
    
    #[error("Configuration error: {0}")]
    ConfigError(String),
    
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
    
    #[error("Serialization error: {0}")]
    SerializationError(String),
    
    #[error("Model not found: {0}")]
    ModelNotFound(String),
}

pub type Result<T> = std::result::Result<T, AssistantError>;
