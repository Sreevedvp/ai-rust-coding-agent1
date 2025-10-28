use crate::llm::ollama::OllamaClient;
use crate::AssistantError;

pub struct EmbeddingService {
    ollama: OllamaClient,
    model: String,
}

impl EmbeddingService {
    pub fn new(ollama: OllamaClient, model: String) -> Self {
        Self { ollama, model }
    }
    
    pub async fn embed_text(&self, text: &str) -> Result<Vec<f32>, AssistantError> {
        self.ollama.embed(&self.model, text).await
    }
    
    pub async fn embed_batch(&self, texts: Vec<String>) -> Result<Vec<Vec<f32>>, AssistantError> {
        let mut embeddings = Vec::new();
        
        for text in texts {
            let embedding = self.embed_text(&text).await?;
            embeddings.push(embedding);
        }
        
        Ok(embeddings)
    }
}

pub fn cosine_similarity(a: &[f32], b: &[f32]) -> f32 {
    let dot: f32 = a.iter().zip(b.iter()).map(|(x, y)| x * y).sum();
    let mag_a: f32 = a.iter().map(|x| x * x).sum::<f32>().sqrt();
    let mag_b: f32 = b.iter().map(|x| x * x).sum::<f32>().sqrt();
    
    if mag_a == 0.0 || mag_b == 0.0 {
        0.0
    } else {
        dot / (mag_a * mag_b)
    }
}

