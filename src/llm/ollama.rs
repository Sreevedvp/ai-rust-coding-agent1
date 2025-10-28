use super::types::*;
use crate::AssistantError;

pub struct OllamaClient {
    base_url: String,
    client: reqwest::Client,
}

impl OllamaClient {
    pub fn new(base_url: String) -> Self {
        Self {
            base_url,
            client: reqwest::Client::new(),
        }
    }
    
    pub async fn chat(
        &self,
        model: &str,
        messages: Vec<Message>,
        temperature: f32,
    ) -> Result<String, AssistantError> {
        let request = ChatRequest {
            model: model.to_string(),
            messages,
            stream: false,
            options: Some(ChatOptions { temperature }),
        };
        
        let response = self
            .client
            .post(format!("{}/api/chat", self.base_url))
            .json(&request)
            .send()
            .await
            .map_err(|e| AssistantError::OllamaError(e.to_string()))?;
        
        if !response.status().is_success() {
            return Err(AssistantError::OllamaError(format!(
                "HTTP {}: {}",
                response.status(),
                response.text().await.unwrap_or_default()
            )));
        }
        
        let chat_response: ChatResponse = response
            .json()
            .await
            .map_err(|e| AssistantError::OllamaError(e.to_string()))?;
        
        Ok(chat_response.message.content)
    }
    
    pub async fn embed(
        &self,
        model: &str,
        text: &str,
    ) -> Result<Vec<f32>, AssistantError> {
        let request = EmbedRequest {
            model: model.to_string(),
            prompt: text.to_string(),
        };
        
        let response = self
            .client
            .post(format!("{}/api/embeddings", self.base_url))
            .json(&request)
            .send()
            .await
            .map_err(|e| AssistantError::OllamaError(e.to_string()))?;
        
        let embed_response: EmbedResponse = response
            .json()
            .await
            .map_err(|e| AssistantError::OllamaError(e.to_string()))?;
        
        Ok(embed_response.embedding)
    }
    
    pub async fn check_model(&self, model: &str) -> Result<bool, AssistantError> {
        let response = self
            .client
            .get(format!("{}/api/tags", self.base_url))
            .send()
            .await
            .map_err(|e| AssistantError::OllamaError(e.to_string()))?;
        
        let text = response.text().await.unwrap_or_default();
        Ok(text.contains(model))
    }
}

