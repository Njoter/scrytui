use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Card {
    pub id: String,
    pub name: String,
    pub mana_cost: Option<String>,
    pub oracle_text: Option<String>,
    pub power: Option<String>,
    pub toughness: Option<String>,
    pub type_line: String,
    pub set_id: String,
    pub set: String,
    pub set_name: String,
    pub rarity: String,
}

#[derive(Debug, Deserialize)]
pub struct CardResponse {
    pub object: String,
    pub total_cards: u32,
    pub has_more: bool,
    #[serde(rename = "data")]
    pub cards: Vec<Card>,
}
