use crate::llm::types::Message;

pub fn build_system_prompt(knowledge_context: &str) -> String {
    format!(
        "You are a helpful personal assistant running locally. \
        Use the following information from your knowledge base to help answer questions:\n\n\
        {}\n\n\
        Be helpful, concise, and friendly. If you don't know something, say so honestly.",
        knowledge_context
    )
}

pub fn format_context(documents: &[String]) -> String {
    if documents.is_empty() {
        "No relevant information in knowledge base.".to_string()
    } else {
        documents.join("\n\n")
    }
}

pub fn build_messages(
    system_prompt: String,
    conversation_history: Vec<Message>,
    user_message: String,
) -> Vec<Message> {
    let mut messages = vec![Message {
        role: "system".to_string(),
        content: system_prompt,
    }];
    
    messages.extend(conversation_history);
    
    messages.push(Message {
        role: "user".to_string(),
        content: user_message,
    });
    
    messages
}

