mod api;
mod models;

use clap::{Parser, Subcommand};
use api::client::ScryfallClient;
use models::card::{Card, ApiResponse};

#[derive(Parser)]
#[command(name = "scrytui")]
#[command(version = "0.1.0")]
#[command(about = "Search Magic: The Gathering cards in your terminal", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    // Search for cards (returns multiple results)
    Search {
        // Search query
        query: String,
        
        // Page number
        #[arg(short, long, default_value_t = 1)]
        page: u32,
    },
    
    // Get exact card by name (returns single card)
    Get {
        // Exact card name
        name: String,
    },
    
    // Fuzzy search (handles typos)
    Fuzzy {
        // Card name (can be approximate)
        name: String,
    },
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();
    let client = ScryfallClient::new();
    
    match cli.command {
        Some(Commands::Get { name }) => {
            println!("Fetching exact card: {}", name);
            
            match client.search_by_exact_name(&name).await {
                Ok(api_response) => {
                    // Returns ApiResponse with total_cards=1
                    println!("Found card!");
                    println!("Total in response: {}", api_response.total_cards);
                    
                    // Extract the single card
                    if let Some(card) = api_response.cards.first() {
                        display_card(card);
                    }
                }
                Err(e) => {
                    eprintln!("❌ Error: {}", e);
                    
                    // Check if it's a 404 (card not found)
                    if let Some(reqwest_err) = e.downcast_ref::<reqwest::Error>() {
                        if reqwest_err.status() == Some(reqwest::StatusCode::NOT_FOUND) {
                            eprintln!("Card '{}' not found. Try fuzzy search?", name);
                        }
                    }
                }
            }
        }
        
        Some(Commands::Search { query, page }) => {
            println!("🔍 Searching for: {}", query);
            println!("📄 Page: {}", page);
            
            // You'll need to implement this method for general search
            // let response = client.search_by_name(&query, page).await?;
            // ...
        }
        
        Some(Commands::Fuzzy { name }) => {
            println!("Fuzzy search for: {}", name);
            
            match client.search_by_fuzzy_name(&name).await {
                Ok(api_response) => {
                    // Returns ApiResponse with total_cards=1
                    println!("Found card!");
                    println!("Total in response: {}", api_response.total_cards);
                    
                    // Extract the single card
                    if let Some(card) = api_response.cards.first() {
                        display_card(card);
                    }
                }
                Err(e) => {
                    eprintln!("❌ Error: {}", e);
                    
                    // Check if it's a 404 (card not found)
                    if let Some(reqwest_err) = e.downcast_ref::<reqwest::Error>() {
                        if reqwest_err.status() == Some(reqwest::StatusCode::NOT_FOUND) {
                            eprintln!("Card '{}' not found.", name);
                        }
                    }
                }
            }
        }
        
        None => {
            println!("No command provided. Use --help for usage.");
        }
    }
    
    Ok(())
}

// Display function for a single card
fn display_card(card: &Card) {
    println!("\n════════════════════════════════════════");
    println!("{}", card.name);
    println!("════════════════════════════════════════");
    
    if let Some(mana_cost) = &card.mana_cost {
        println!("Mana cost: {}", mana_cost);
    }
    
    println!("Type: {}", card.type_line);
    
    if let Some(oracle_text) = &card.oracle_text {
        println!("\n{}", oracle_text);
    }
    
    if let Some(power) = &card.power {
        if let Some(toughness) = &card.toughness {
            println!("\nPower/Toughness: {}/{}", power, toughness);
        }
    }
    
    println!("Set: {} ({})", card.set_name, card.set);
    println!("Rarity: {}", card.rarity);
}
