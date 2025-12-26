use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "scrytui")]
#[command(version = "0.1.0")]
#[command(about = "Search Magic: The Gathering cards in your terminal", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand)]
pub enum Commands {
    
    Search { // Search for cards (returns multiple results)
        query: String,
        #[arg(short, long, default_value_t = 1)]
        page: u32,
    },
    
    Get { // Get exact card by name (returns single card)
        name: String,
    },
    
    Fuzzy { // Fuzzy search (handles typos)
        name: String,
    },
}
