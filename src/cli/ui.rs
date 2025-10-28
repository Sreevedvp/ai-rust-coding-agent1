use crate::agent::assistant::Assistant;
use colored::*;
use std::io::{self, Write};
use anyhow::Result;

pub struct UI {
    assistant: Assistant,
}

impl UI {
    pub fn new(assistant: Assistant) -> Self {
        Self { assistant }
    }
    
    pub async fn run_interactive(mut self) -> Result<()> {
        println!("\n{}", "📋 Commands:".bold());
        println!("  • Type your message to chat");
        println!("  • 'learn: <text>' - Teach new information");
        println!("  • 'file: <path>' - Learn from a file");
        println!("  • 'save' - Save conversation");
        println!("  • 'clear' - Clear history");
        println!("  • 'quit' - Exit");
        println!("{}\n", "─".repeat(60));
        
        loop {
            print!("{} ", "You:".green().bold());
            io::stdout().flush()?;
            
            let mut input = String::new();
            io::stdin().read_line(&mut input)?;
            let input = input.trim();
            
            if input.is_empty() {
                continue;
            }
            
            match input.to_lowercase().as_str() {
                "quit" | "exit" => {
                    println!("\n{} {}", "👋".yellow(), "Goodbye!".bold());
                    break;
                }
                "clear" => {
                    self.assistant.clear_history().await?;
                    println!("{} {}\n", "✅".green(), "History cleared");
                }
                "save" => {
                    self.assistant.save_conversation(None).await?;
                    println!("{} {}\n", "✅".green(), "Conversation saved");
                }
                _ if input.starts_with("learn:") => {
                    let text = &input[6..].trim();
                    match self.assistant.learn_text(text, "cli").await {
                        Ok(_) => println!("{} {}\n", "✅".green(), "Learned new information"),
                        Err(e) => println!("{} {}\n", "❌".red(), e),
                    }
                }
                _ if input.starts_with("file:") => {
                    let filepath = &input[5..].trim();
                    match self.assistant.learn_file(filepath).await {
                        Ok(_) => println!("{} Learned from {}\n", "✅".green(), filepath),
                        Err(e) => println!("{} {}\n", "❌".red(), e),
                    }
                }
                _ => {
                    print!("{} ", "🤔 Thinking...".dimmed());
                    io::stdout().flush()?;
                    
                    match self.assistant.chat(input).await {
                        Ok(response) => {
                            print!("\r{}\r", " ".repeat(20));
                            println!("{} {}\n", "🤖 Assistant:".cyan().bold(), response);
                        }
                        Err(e) => {
                            print!("\r{}\r", " ".repeat(20));
                            println!("{} {}\n", "❌ Error:".red(), e);
                            println!("Make sure Ollama is running: {}\n", "ollama serve".yellow());
                        }
                    }
                }
            }
        }
        
        Ok(())
    }
}

