use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use anyhow::Result;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Settings {
    pub ollama_host: String,
    pub ollama_model: String,
    pub embedding_model: String,
    pub temperature: f32,
    pub data_dir: PathBuf,
    pub knowledge_dir: PathBuf,
    pub conversations_dir: PathBuf,
    pub chunk_size: usize,
    pub chunk_overlap: usize,
    pub max_history: usize,
    pub retrieval_k: usize,
}

impl Settings {
    pub fn new() -> Result<Self> {
        let ollama_host = std::env::var("OLLAMA_HOST")
            .unwrap_or_else(|_| "http://localhost:11434".to_string());
        
        let ollama_model = std::env::var("OLLAMA_MODEL")
            .unwrap_or_else(|_| "qwen2.5:7b".to_string());
        
        let embedding_model = std::env::var("OLLAMA_EMBEDDING_MODEL")
            .unwrap_or_else(|_| "nomic-embed-text".to_string());
        
        let temperature = std::env::var("OLLAMA_TEMPERATURE")
            .unwrap_or_else(|_| "0.7".to_string())
            .parse()
            .unwrap_or(0.7);
        
        let data_dir = PathBuf::from(
            std::env::var("DATA_DIR").unwrap_or_else(|_| "./data".to_string())
        );
        
        Ok(Self {
            ollama_host,
            ollama_model,
            embedding_model,
            temperature,
            knowledge_dir: data_dir.join("knowledge_base"),
            conversations_dir: data_dir.join("conversations"),
            data_dir,
            chunk_size: 500,
            chunk_overlap: 50,
            max_history: 6,
            retrieval_k: 3,
        })
    }
    
    pub fn ensure_dirs(&self) -> Result<()> {
        std::fs::create_dir_all(&self.data_dir)?;
        std::fs::create_dir_all(&self.knowledge_dir)?;
        std::fs::create_dir_all(&self.conversations_dir)?;
        Ok(())
    }
}

