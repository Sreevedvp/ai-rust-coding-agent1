use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message {
    pub role: String,
    pub content: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ChatRequest {
    pub model: String,
    pub messages: Vec<Message>,
    pub stream: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub options: Option<ChatOptions>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ChatOptions {
    pub temperature: f32,
}

#[derive(Debug, Deserialize)]
pub struct ChatResponse {
    pub message: Message,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EmbedRequest {
    pub model: String,
    pub prompt: String,
}

#[derive(Debug, Deserialize)]
pub struct EmbedResponse {
    pub embedding: Vec<f32>,
}
