use super::documents::Document;
use super::embeddings::{EmbeddingService, cosine_similarity};
use crate::AssistantError;
use std::path::PathBuf;

pub struct VectorStore {
    documents: Vec<Document>,
    storage_path: PathBuf,
    embedding_service: EmbeddingService,
}

impl VectorStore {
    pub fn new(storage_path: PathBuf, embedding_service: EmbeddingService) -> Self {
        let mut store = Self {
            documents: Vec::new(),
            storage_path,
            embedding_service,
        };
        
        let _ = store.load();
        store
    }
    
    pub async fn add_document(&mut self, mut doc: Document) -> Result<(), AssistantError> {
        let embedding = self.embedding_service.embed_text(&doc.content).await?;
        doc.embedding = Some(embedding);
        self.documents.push(doc);
        self.save()?;
        Ok(())
    }
    
    pub async fn add_documents(&mut self, docs: Vec<Document>) -> Result<(), AssistantError> {
        for doc in docs {
            self.add_document(doc).await?;
        }
        Ok(())
    }
    
    pub async fn search(&self, query: &str, k: usize) -> Result<Vec<Document>, AssistantError> {
        let query_embedding = self.embedding_service.embed_text(query).await?;
        
        let mut scored_docs: Vec<(f32, Document)> = self
            .documents
            .iter()
            .filter_map(|doc| {
                doc.embedding.as_ref().map(|emb| {
                    let score = cosine_similarity(&query_embedding, emb);
                    (score, doc.clone())
                })
            })
            .collect();
        
        scored_docs.sort_by(|a, b| b.0.partial_cmp(&a.0).unwrap());
        
        Ok(scored_docs.into_iter().take(k).map(|(_, doc)| doc).collect())
    }
    
    pub fn count(&self) -> usize {
        self.documents.len()
    }
    
    pub fn export(&self) -> Result<Vec<Document>, AssistantError> {
        Ok(self.documents.clone())
    }
    
    fn save(&self) -> Result<(), AssistantError> {
        let json = serde_json::to_string_pretty(&self.documents)
            .map_err(|e| AssistantError::SerializationError(e.to_string()))?;
        
        std::fs::write(&self.storage_path, json)?;
        Ok(())
    }
    
    fn load(&mut self) -> Result<(), AssistantError> {
        if !self.storage_path.exists() {
            return Ok(());
        }
        
        let json = std::fs::read_to_string(&self.storage_path)?;
        self.documents = serde_json::from_str(&json)
            .map_err(|e| AssistantError::SerializationError(e.to_string()))?;
        
        Ok(())
    }
}

