use crate::memory::conversation::Message;
use crate::{AssistantError, Result};
use std::path::Path;

pub use crate::memory::conversation::ConversationManager;

pub fn save_conversation(messages: &[Message], path: impl AsRef<Path>) -> Result<()> {
    let json = serde_json::to_string_pretty(messages)
        .map_err(|e| AssistantError::SerializationError(e.to_string()))?;
    
    std::fs::write(path, json)
        .map_err(|e| AssistantError::IoError(e))?;
    
    Ok(())
}

pub fn load_conversation(path: impl AsRef<Path>) -> Result<Vec<Message>> {
    let content = std::fs::read_to_string(path)
        .map_err(|e| AssistantError::IoError(e))?;
    
    let messages: Vec<Message> = serde_json::from_str(&content)
        .map_err(|e| AssistantError::SerializationError(e.to_string()))?;
    
    Ok(messages)
}