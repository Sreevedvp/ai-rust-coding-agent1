use std::path::Path;
use anyhow::Result;
use walkdir::WalkDir;

pub fn read_file(path: &str) -> Result<String> {
    Ok(std::fs::read_to_string(path)?)
}

pub fn read_directory(path: &str, extensions: &[&str]) -> Result<Vec<String>> {
    let mut contents = Vec::new();
    
    for entry in WalkDir::new(path).follow_links(true) {
        let entry = entry?;
        let path = entry.path();
        
        if path.is_file() {
            if let Some(ext) = path.extension() {
                if extensions.is_empty() || extensions.contains(&ext.to_str().unwrap_or("")) {
                    if let Ok(content) = std::fs::read_to_string(path) {
                        contents.push(content);
                    }
                }
            }
        }
    }
    
    Ok(contents)
}

pub fn is_text_file(path: &Path) -> bool {
    if let Some(ext) = path.extension() {
        let ext = ext.to_str().unwrap_or("");
        matches!(ext, "txt" | "md" | "rs" | "py" | "js" | "json" | "toml" | "yaml" | "yml")
    } else {
        false
    }
}
