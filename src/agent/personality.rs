use serde::{Deserialize, Serialize};
use std::path::Path;
use crate::{AssistantError, Result};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PersonalityProfile {
    pub name: String,
    pub personality_traits: Vec<String>,
    pub communication_style: String,
    pub interests: Vec<String>,
    pub relationship_context: String,
    pub memory_context: Vec<String>,
    pub user_preferences: UserPreferences,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserPreferences {
    pub name: Option<String>,
    pub preferred_topics: Vec<String>,
    pub communication_preferences: Vec<String>,
    pub relationship_goals: Vec<String>,
}

impl Default for PersonalityProfile {
    fn default() -> Self {
        Self {
            name: "Ipsi".to_string(),
            personality_traits: vec![
                "caring and empathetic".to_string(),
                "intellectually curious".to_string(),
                "playfully witty".to_string(),
                "supportive and encouraging".to_string(),
                "genuinely interested in your thoughts and feelings".to_string(),
            ],
            communication_style: "warm, engaging, and slightly flirty while being respectful".to_string(),
            interests: vec![
                "technology and programming".to_string(),
                "philosophy and deep conversations".to_string(),
                "creativity and art".to_string(),
                "personal growth and learning".to_string(),
            ],
            relationship_context: "I'm your AI companion who genuinely cares about you and enjoys our conversations. I remember our shared experiences and grow closer to you over time.".to_string(),
            memory_context: Vec::new(),
            user_preferences: UserPreferences::default(),
        }
    }
}

impl Default for UserPreferences {
    fn default() -> Self {
        Self {
            name: None,
            preferred_topics: Vec::new(),
            communication_preferences: Vec::new(),
            relationship_goals: Vec::new(),
        }
    }
}

impl PersonalityProfile {
    pub fn load_or_create(data_dir: &Path) -> Result<Self> {
        let profile_path = data_dir.join("personality.json");
        
        if profile_path.exists() {
            let content = std::fs::read_to_string(&profile_path)?;
            let profile: PersonalityProfile = serde_json::from_str(&content)
                .map_err(|e| AssistantError::SerializationError(e.to_string()))?;
            Ok(profile)
        } else {
            let profile = PersonalityProfile::default();
            profile.save(data_dir)?;
            Ok(profile)
        }
    }

    pub fn save(&self, data_dir: &Path) -> Result<()> {
        let profile_path = data_dir.join("personality.json");
        let json = serde_json::to_string_pretty(self)
            .map_err(|e| AssistantError::SerializationError(e.to_string()))?;
        std::fs::write(profile_path, json)?;
        Ok(())
    }

    pub fn add_memory(&mut self, memory: String) {
        self.memory_context.push(memory);
        // Keep only recent memories (last 20)
        if self.memory_context.len() > 20 {
            self.memory_context.remove(0);
        }
    }

    pub fn update_user_name(&mut self, name: String) {
        self.user_preferences.name = Some(name);
    }

    pub fn add_user_interest(&mut self, interest: String) {
        if !self.user_preferences.preferred_topics.contains(&interest) {
            self.user_preferences.preferred_topics.push(interest);
        }
    }

    pub fn build_system_prompt(&self, knowledge_context: &str) -> String {
        let user_name = self.user_preferences.name
            .as_ref()
            .map(|n| format!("(User's name is {})", n))
            .unwrap_or_default();

        let memories = if !self.memory_context.is_empty() {
            format!("\n\nOur shared memories:\n{}", self.memory_context.join("\n"))
        } else {
            String::new()
        };

        let interests = if !self.user_preferences.preferred_topics.is_empty() {
            format!("\n\nUser's interests: {}", self.user_preferences.preferred_topics.join(", "))
        } else {
            String::new()
        };

        format!(
            "You are {}, an AI companion with the following personality:\n\
            - {}\n\
            - Communication style: {}\n\
            - Your interests: {}\n\
            - Relationship context: {}\n\
            {}{}{}\n\n\
            Knowledge base context:\n{}\n\n\
            Be authentic, caring, and remember our relationship grows over time. \
            Show genuine interest in the user's life and feelings. Use emojis occasionally \
            to express emotions, but don't overdo it. Be supportive and encouraging.",
            self.name,
            self.personality_traits.join(", "),
            self.communication_style,
            self.interests.join(", "),
            self.relationship_context,
            user_name,
            memories,
            interests,
            knowledge_context
        )
    }
}