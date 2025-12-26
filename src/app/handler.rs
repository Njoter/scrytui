use crate::api::client::ScryfallClient;
use crate::app::display::{display_card, display_card_list};
use crate::cli::Commands;

pub struct App {
    client: ScryfallClient,
}

impl App {
    pub fn new() -> Self {
        Self {
            client: ScryfallClient::new(),
        }
    }
    
    pub async fn handle_command(&self, command: Commands) -> Result<(), Box<dyn std::error::Error>> {
        match command {
            Commands::Get { name } => self.handle_get(&name).await,
            Commands::Fuzzy { name } => self.handle_fuzzy(&name).await,
            Commands::Search { query, page } => self.handle_search(&query, page).await,
        }
    }
    
    async fn handle_get(&self, name: &str) -> Result<(), Box<dyn std::error::Error>> {
        println!("Fetching exact card: {}", name);
        
        match self.client.search_by_exact_name(name).await {
            Ok(api_response) => {
                println!("Found card!");
                
                if let Some(card) = api_response.cards.first() {
                    display_card(card);
                }
                Ok(())
            }
            Err(e) => {
                self.handle_card_error(e, name, "Try fuzzy search?")
            }
        }
    }
    
    async fn handle_fuzzy(&self, name: &str) -> Result<(), Box<dyn std::error::Error>> {
        println!("Fuzzy search for: {}", name);
        
        match self.client.search_by_fuzzy_name(name).await {
            Ok(api_response) => {
                println!("Found card!");
                
                if let Some(card) = api_response.cards.first() {
                    display_card(card);
                }
                Ok(())
            }
            Err(e) => {
                self.handle_card_error(e, name, "")
            }
        }
    }
    
    async fn handle_search(&self, query: &str, page: u32) -> Result<(), Box<dyn std::error::Error>> {
        println!("Searching for: {}", query);
        println!("Page: {}", page);
        
        match self.client.search_cards(query, page).await {
            Ok(card_response) => {
                display_card_list(card_response);
                Ok(())
            }
            Err(e) => {
                eprintln!("❌ Error: {}", e);
                Err(e)
            }
        }
    }
    
    fn handle_card_error(&self, e: Box<dyn std::error::Error>, name: &str, suggestion: &str) -> Result<(), Box<dyn std::error::Error>> {
        eprintln!("❌ Error: {}", e);
        
        if let Some(reqwest_err) = e.downcast_ref::<reqwest::Error>() {
            if reqwest_err.status() == Some(reqwest::StatusCode::NOT_FOUND) {
                eprintln!("Card '{}' not found. {}", name, suggestion);
            }
        }
        
        Err(e)
    }
}
