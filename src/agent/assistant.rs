use crate::config::settings::Settings;
use crate::knowledge::documents::{Document, chunk_text};
use crate::knowledge::embeddings::EmbeddingService;
use crate::knowledge::vectorstore::VectorStore;
use crate::llm::ollama::OllamaClient;
use crate::memory::storage::ConversationManager;
use crate::{AssistantError, Result};
use super::chain::{build_system_prompt, format_context, build_messages};

use chrono::Utc;

pub struct Assistant {
    settings: Settings,
    ollama: OllamaClient,
    vectorstore: VectorStore,
    conversation: ConversationManager,
}

impl Assistant {
    pub async fn new(settings: Settings) -> Result<Self> {
        // settings.ensure_dirs()?;
        
        let ollama = OllamaClient::new(settings.ollama_host.clone());
        
        // Check if model exists
        if !ollama.check_model(&settings.ollama_model).await? {
            return Err(AssistantError::ModelNotFound(settings.ollama_model.clone()));
        }
        
        let embedding_service = EmbeddingService::new(
            OllamaClient::new(settings.ollama_host.clone()),
            settings.embedding_model.clone(),
        );
        
        let storage_path = settings.knowledge_dir.join("documents.json");
        let vectorstore = VectorStore::new(storage_path, embedding_service);
        
        let conversation = ConversationManager::new(settings.max_history);
        
        Ok(Self {
            settings,
            ollama,
            vectorstore,
            conversation,
        })
    }
    
    pub async fn chat(&mut self, user_message: &str) -> Result<String> {
        // Search knowledge base
        let relevant_docs = self.vectorstore.search(user_message, self.settings.retrieval_k).await?;
        let context_strings: Vec<String> = relevant_docs.iter()
            .map(|doc| doc.content.clone())
            .collect();
        let context = format_context(&context_strings);
        
        // Build prompt
        let system_prompt = build_system_prompt(&context);
        let history = self.conversation.get_recent_llm_messages();
        let messages = build_messages(system_prompt, history, user_message.to_string());
        
        // Get response
        let response = self.ollama.chat(
            &self.settings.ollama_model,
            messages,
            self.settings.temperature,
        ).await?;
        
        // Update conversation
        self.conversation.add_message("user".to_string(), user_message.to_string());
        self.conversation.add_message("assistant".to_string(), response.clone());
        
        Ok(response)
    }
    
    pub async fn learn_text(&mut self, text: &str, source: &str) -> Result<()> {
        let chunks = chunk_text(text, self.settings.chunk_size, self.settings.chunk_overlap);
        
        let documents: Vec<Document> = chunks
            .into_iter()
            .map(|chunk| Document::new(chunk, source.to_string()))
            .collect();
        
        self.vectorstore.add_documents(documents).await?;
        
        Ok(())
    }
    
    pub async fn learn_file(&mut self, filepath: &str) -> Result<()> {
        let content = std::fs::read_to_string(filepath)
            .map_err(|e| AssistantError::IoError(e))?;
        
        self.learn_text(&content, filepath).await
    }
    
    pub async fn export_knowledge(&self, output_path: &str) -> Result<()> {
        let docs = self.vectorstore.export()?;
        let json = serde_json::to_string_pretty(&docs)
            .map_err(|e| AssistantError::SerializationError(e.to_string()))?;
        
        std::fs::write(output_path, json)?;
        Ok(())
    }
    
    pub async fn clear_history(&mut self) -> Result<()> {
        self.conversation.clear();
        Ok(())
    }
    
    pub async fn save_conversation(&self, name: Option<String>) -> Result<()> {
        let filename = name.unwrap_or_else(|| {
            format!("conversation_{}.json", Utc::now().format("%Y%m%d_%H%M%S"))
        });
        
        let path = self.settings.conversations_dir.join(filename);
        let entries = self.conversation.export();
        // storage::save_conversation(&entries, path)?;
        
        Ok(())
    }
    
    pub async fn get_info(&self) -> Result<AssistantInfo> {
        let ollama_available = self.ollama.check_model(&self.settings.ollama_model).await.unwrap_or(false);
        
        Ok(AssistantInfo {
            model: self.settings.ollama_model.clone(),
            embedding_model: self.settings.embedding_model.clone(),
            knowledge_count: self.vectorstore.count(),
            conversation_count: self.conversation.count(),
            ollama_available,
            data_dir: self.settings.data_dir.display().to_string(),
        })
    }
}

pub struct AssistantInfo {
    pub model: String,
    pub embedding_model: String,
    pub knowledge_count: usize,
    pub conversation_count: usize,
    pub ollama_available: bool,
    pub data_dir: String,
}

