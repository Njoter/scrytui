use reqwest::Client;
use reqwest::header::{HeaderMap, HeaderValue, USER_AGENT, ACCEPT};

use crate::models::card::{ApiResponse, Card};

pub struct ScryfallClient {
    client: Client,
    base_url: String,
}

impl ScryfallClient {
    pub fn new() -> Self {
        let mut headers = HeaderMap::new();

        headers.insert(
            ACCEPT,
            HeaderValue::from_static("application/json;q=0.9,*/*;q=0.8")
        );

        headers.insert(
            USER_AGENT,
            HeaderValue::from_static("scrytui/0.1.0")
        );

        let client = Client::builder()
            .default_headers(headers)
            .build()
            .expect("Failed to build HTTP client");

        Self {
            client,
            base_url: String::from("https://api.scryfall.com"),
        }
    }

    pub async fn search_by_exact_name(&self, name: &str) -> Result<ApiResponse, Box<dyn std::error::Error>> {
        self.fetch_single_card("exact", name).await
    }

    pub async fn search_by_fuzzy_name(&self, name: &str) -> Result<ApiResponse, Box<dyn std::error::Error>> {
        self.fetch_single_card("fuzzy", name).await
    }

    async fn fetch_single_card(&self, search_type: &str,  name: &str) -> Result<ApiResponse, Box<dyn std::error::Error>> {
        let url = format!("{}/cards/named", self.base_url);

        let response = self.client
            .get(&url)
            .query(&[(&search_type, &name)])
            .send()
            .await?;

        let response = response.error_for_status()?;
        let body = response.text().await?;

        let card: Card = serde_json::from_str(&body)?;

        let api_response = ApiResponse {
            object: "list".to_string(),
            total_cards: 1,
            has_more: false,
            cards: vec![card],
        };

        Ok(api_response)
    }
}
