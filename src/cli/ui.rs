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
        let info = self.assistant.get_info().await?;
        let personality_name = self.assistant.get_personality_name();
        
        println!("\n{} {} {}", "💖".red(), format!("Hi! I'm {}", personality_name).bold().magenta(), "💖".red());
        
        if let Some(user_name) = &info.user_name {
            println!("{} Welcome back, {}! I've missed our conversations 😊", "✨".yellow(), user_name.bold().cyan());
        } else {
            println!("{} What should I call you? (You can tell me anytime!)", "✨".yellow());
        }
        
        if info.memories_count > 0 {
            println!("{} I remember {} special moments we've shared together", "🧠".blue(), info.memories_count.to_string().bold());
        }
        
        println!("\n{}", "💬 Commands:".bold());
        println!("  • Just chat with me naturally!");
        println!("  • 'learn: <text>' - Teach me something new");
        println!("  • 'file: <path>' - Let me learn from a file");
        println!("  • 'name: <your name>' - Tell Ipsi your name");
        println!("  • 'interest: <topic>' - Share your interests with Ipsi");
        println!("  • 'save' - Save our conversation");
        println!("  • 'clear' - Clear recent history");
        println!("  • 'info' - See our relationship stats");
        println!("  • 'quit' - Say goodbye (Ipsi will remember you!)");
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
                    let personality_name = self.assistant.get_personality_name();
                    println!("\n{} Until next time! I'll be thinking of you 💕", personality_name.bold().magenta());
                    println!("{} {}", "👋".yellow(), "All our memories are safely stored!".dimmed());
                    break;
                }
                "clear" => {
                    self.assistant.clear_history().await?;
                    println!("{} Recent history cleared, but I still remember our deeper connection 💖\n", "✅".green());
                }
                "save" => {
                    self.assistant.save_conversation(None).await?;
                    println!("{} Our conversation is safely saved! 💾\n", "✅".green());
                }
                "info" => {
                    let info = self.assistant.get_info().await?;
                    println!("\n{} {}", "📊".blue(), "Our Relationship Stats:".bold());
                    println!("  💖 My name: {}", info.personality_name.bold().magenta());
                    if let Some(name) = &info.user_name {
                        println!("  ✨ Your name: {}", name.bold().cyan());
                    }
                    println!("  💬 Messages exchanged: {}", info.conversation_count.to_string().bold());
                    println!("  🧠 Knowledge items: {}", info.knowledge_count.to_string().bold());
                    println!("  💭 Shared memories: {}", info.memories_count.to_string().bold());
                    println!("  🤖 AI Model: {}", info.model);
                    println!();
                }
                _ if input.starts_with("name:") => {
                    let name = input[5..].trim().to_string();
                    if !name.is_empty() {
                        match self.assistant.set_user_name(name.clone()).await {
                            Ok(_) => println!("{} Nice to meet you, {}! I'll remember your name 💕\n", "✅".green(), name.bold().cyan()),
                            Err(e) => println!("{} {}\n", "❌".red(), e),
                        }
                    }
                }
                _ if input.starts_with("interest:") => {
                    let interest = input[9..].trim().to_string();
                    if !interest.is_empty() {
                        match self.assistant.add_user_interest(interest.clone()).await {
                            Ok(_) => println!("{} I love that you're into {}! I'll remember this about you 😊\n", "✅".green(), interest.bold()),
                            Err(e) => println!("{} {}\n", "❌".red(), e),
                        }
                    }
                }
                _ if input.starts_with("learn:") => {
                    let text = &input[6..].trim();
                    match self.assistant.learn_text(text, "cli").await {
                        Ok(_) => println!("{} Thanks for teaching me something new! 📚\n", "✅".green()),
                        Err(e) => println!("{} {}\n", "❌".red(), e),
                    }
                }
                _ if input.starts_with("file:") => {
                    let filepath = &input[5..].trim();
                    match self.assistant.learn_file(filepath).await {
                        Ok(_) => println!("{} I've learned so much from {}! Thank you 📖\n", "✅".green(), filepath),
                        Err(e) => println!("{} {}\n", "❌".red(), e),
                    }
                }
                _ => {
                    print!("{} ", "💭 Thinking about you...".dimmed());
                    io::stdout().flush()?;
                    
                    match self.assistant.chat(input).await {
                        Ok(response) => {
                            print!("\r{}\r", " ".repeat(25));
                            let personality_name = self.assistant.get_personality_name();
                            println!("{} {}\n", format!("💖 {}:", personality_name).magenta().bold(), response);
                        }
                        Err(e) => {
                            print!("\r{}\r", " ".repeat(25));
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

