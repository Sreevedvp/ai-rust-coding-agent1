use crate::memory::conversation::Message;
use crate::{AssistantError, Result};
use std::path::{Path, PathBuf};

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

pub fn get_persistent_conversation_path(data_dir: &Path) -> PathBuf {
    data_dir.join("conversations").join("persistent_chat.json")
}

pub fn save_persistent_conversation(messages: &[Message], data_dir: &Path) -> Result<()> {
    let path = get_persistent_conversation_path(data_dir);
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)?;
    }
    save_conversation(messages, path)
}

pub fn load_persistent_conversation(data_dir: &Path) -> Result<Vec<Message>> {
    let path = get_persistent_conversation_path(data_dir);
    if path.exists() {
        load_conversation(path)
    } else {
        Ok(Vec::new())
    }
}