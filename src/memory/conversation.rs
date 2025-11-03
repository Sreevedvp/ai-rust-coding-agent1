use serde::{Deserialize, Serialize};
use std::collections::VecDeque;
use crate::llm::types::Message as LlmMessage;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message {
    pub role: String,
    pub content: String,
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

impl Message {
    pub fn new(role: String, content: String) -> Self {
        Self {
            role,
            content,
            timestamp: chrono::Utc::now(),
        }
    }

    pub fn to_llm_message(&self) -> LlmMessage {
        LlmMessage {
            role: self.role.clone(),
            content: self.content.clone(),
        }
    }
}

#[derive(Debug)]
pub struct ConversationManager {
    messages: VecDeque<Message>,
    max_history: usize,
}

impl ConversationManager {
    pub fn new(max_history: usize) -> Self {
        Self {
            messages: VecDeque::new(),
            max_history,
        }
    }

    pub fn new_with_history(max_history: usize, history: Vec<Message>) -> Self {
        let mut messages = VecDeque::new();
        for msg in history {
            messages.push_back(msg);
        }
        
        // Keep only the most recent messages
        while messages.len() > max_history {
            messages.pop_front();
        }
        
        Self {
            messages,
            max_history,
        }
    }

    pub fn add_message(&mut self, role: String, content: String) {
        let message = Message::new(role, content);
        self.messages.push_back(message);
        
        // Keep only the most recent messages
        while self.messages.len() > self.max_history {
            self.messages.pop_front();
        }
    }

    pub fn get_recent_messages(&self) -> Vec<Message> {
        self.messages.iter().cloned().collect()
    }

    pub fn get_recent_llm_messages(&self) -> Vec<LlmMessage> {
        self.messages.iter().map(|msg| msg.to_llm_message()).collect()
    }

    pub fn clear(&mut self) {
        self.messages.clear();
    }

    pub fn count(&self) -> usize {
        self.messages.len()
    }

    pub fn export(&self) -> Vec<Message> {
        self.messages.iter().cloned().collect()
    }
}