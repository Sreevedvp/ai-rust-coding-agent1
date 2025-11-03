use crate::config::settings::Settings;
use crate::knowledge::documents::{Document, chunk_text};
use crate::knowledge::embeddings::EmbeddingService;
use crate::knowledge::vectorstore::VectorStore;
use crate::llm::ollama::OllamaClient;
use crate::memory::storage::{ConversationManager, load_persistent_conversation, save_persistent_conversation};
use crate::agent::personality::PersonalityProfile;
use crate::{AssistantError, Result};
use super::chain::{format_context, build_messages};

use chrono::Utc;

pub struct Assistant {
    settings: Settings,
    ollama: OllamaClient,
    vectorstore: VectorStore,
    conversation: ConversationManager,
    personality: PersonalityProfile,
}

impl Assistant {
    pub async fn new(settings: Settings) -> Result<Self> {
        settings.ensure_dirs().map_err(|e| AssistantError::ConfigError(e.to_string()))?;
        
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
        
        // Load personality profile
        let personality = PersonalityProfile::load_or_create(&settings.data_dir)?;
        
        // Load persistent conversation history
        let history = load_persistent_conversation(&settings.data_dir)?;
        let conversation = ConversationManager::new_with_history(settings.max_history, history);
        
        Ok(Self {
            settings,
            ollama,
            vectorstore,
            conversation,
            personality,
        })
    }
    
    pub async fn chat(&mut self, user_message: &str) -> Result<String> {
        // Search knowledge base
        let relevant_docs = self.vectorstore.search(user_message, self.settings.retrieval_k).await?;
        let context_strings: Vec<String> = relevant_docs.iter()
            .map(|doc| doc.content.clone())
            .collect();
        let context = format_context(&context_strings);
        
        // Build personalized system prompt
        let system_prompt = self.personality.build_system_prompt(&context);
        let history = self.conversation.get_recent_llm_messages();
        let messages = build_messages(system_prompt, history, user_message.to_string());
        
        // Get response
        let response = self.ollama.chat(
            &self.settings.ollama_model,
            messages,
            self.settings.temperature,
        ).await?;
        
        // Update conversation and save persistently
        self.conversation.add_message("user".to_string(), user_message.to_string());
        self.conversation.add_message("assistant".to_string(), response.clone());
        
        // Save conversation to disk
        let all_messages = self.conversation.export();
        save_persistent_conversation(&all_messages, &self.settings.data_dir)?;
        
        // Learn from the conversation (add to memory context)
        if user_message.len() > 10 {
            let memory = format!("User said: {} | I responded: {}", 
                user_message.chars().take(100).collect::<String>(),
                response.chars().take(100).collect::<String>()
            );
            self.personality.add_memory(memory);
            self.personality.save(&self.settings.data_dir)?;
        }
        
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
        
        let _path = self.settings.conversations_dir.join(filename);
        let _entries = self.conversation.export();
        // storage::save_conversation(&entries, path)?;
        
        Ok(())
    }
    
    pub async fn set_user_name(&mut self, name: String) -> Result<()> {
        self.personality.update_user_name(name);
        self.personality.save(&self.settings.data_dir)?;
        Ok(())
    }

    pub async fn add_user_interest(&mut self, interest: String) -> Result<()> {
        self.personality.add_user_interest(interest);
        self.personality.save(&self.settings.data_dir)?;
        Ok(())
    }

    pub fn get_personality_name(&self) -> &str {
        &self.personality.name
    }

    pub fn get_user_name(&self) -> Option<&String> {
        self.personality.user_preferences.name.as_ref()
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
            personality_name: self.personality.name.clone(),
            user_name: self.personality.user_preferences.name.clone(),
            memories_count: self.personality.memory_context.len(),
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
    pub personality_name: String,
    pub user_name: Option<String>,
    pub memories_count: usize,
}

