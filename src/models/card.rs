use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct Card {
    pub id: String,
    pub name: String,
}

#[derive(Debug, Deserialize)]
pub struct ApiResponse {
    pub object: String,
    pub total_cards: u32,
    pub has_more: bool,
    #[serde(rename = "data")]
    pub cards: Vec<Card>
}
