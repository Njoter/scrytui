mod api;
mod models;

use clap::Parser;
use api::client::ScryfallClient;
use models::card::ApiResponse;

#[derive(Parser)]
#[command(name = "scrytui")]
#[command(version = "0.1.0")]
#[command(about = "Search Magic: The Gathering cards in your terminal", long_about = None)]
struct Cli {
    search_term: String,

    // Show results as JSON
    #[arg(short, long)]
    json: bool,
    
    // Include card images in output
    #[arg(short, long)]
    images: bool,

    #[arg(short, long, default_value_t = 1)]
    page: u32,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    println!("Searching for: {}", cli.search_term);

    if cli.images {
        println!("Note: Image support coming soon!");
    }

    let api_client = ScryfallClient::new();
    let json_string = api_client
        .search_by_name(&cli.search_term, cli.page)
        .await?;

    if cli.json {
        println!("{}", json_string);
    } else {
        match serde_json::from_str::<ApiResponse>(&json_string) {
            Ok(response) => {
                println!("Found {} cards:", response.total_cards);
                for card in response.cards {
                    println!("{}", card.name);
                }
            }
            Err(e) => {
                eprintln!("Failed to parse JSON: {}", e);
                eprintln!("Showin raw JSON instead:");
                println!("{}", &json_string[..json_string.len().min(500)]);
            }
        }
    }

    Ok(())
}
