use reqwest::Client;
use reqwest::header::{HeaderMap, HeaderValue, USER_AGENT, ACCEPT};

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

    pub async fn search_by_name(&self, name: &str, page: u32) -> Result<String, reqwest::Error> {
        let url = format!("{}/cards/search", self.base_url);

        let response = self.client
            .get(&url)
            .query(&[("q", format!("name:\"{}\"", name)), ("page", page.to_string()), ])
            .send()
            .await?;

        // If 2xx: return Ok(original response)
        // If error: return Err(reqwest::Error) with status info
        let response = response.error_for_status()?;

        let body = response.text().await?;
        Ok(body)
    }
}
