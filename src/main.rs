use assistant_agent::{
    agent::assistant::Assistant,
    cli::ui::UI,
    config::settings::Settings,
};
use colored::*;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    println!("{}", "💖 Starting Your AI Companion...".bold().magenta());
    
    // Load settings
    let settings = Settings::new()?;
    settings.ensure_dirs()?;
    
    println!("{} Connecting to Ollama at {}", "🔗".yellow(), settings.ollama_host);
    println!("{} Using model: {}", "🤖".blue(), settings.ollama_model);
    
    // Initialize assistant
    match Assistant::new(settings).await {
        Ok(assistant) => {
            println!("{} Your companion is ready to chat! 💕\n", "✅".green());
            
            // Start interactive UI
            let ui = UI::new(assistant);
            ui.run_interactive().await?;
        }
        Err(e) => {
            println!("{} Failed to initialize assistant: {}", "❌".red(), e);
            println!("\n{}", "💡 Make sure Ollama is running:".yellow());
            println!("   {}", "ollama serve".bold());
            println!("   {}", "ollama pull qwen2.5:7b".bold());
            println!("   {}", "ollama pull nomic-embed-text".bold());
        }
    }
    
    Ok(())
}
