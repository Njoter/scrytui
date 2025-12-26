use crate::models::card::{Card, CardResponse};

pub fn display_card_list(response: CardResponse) {
    println!("Found {} cards", response.total_cards);

    for (i, card) in response.cards.iter().enumerate() {
        let mana_cost = card.mana_cost.as_deref().unwrap_or("");
        println!("{}. {} {}", i + 1, card.name, mana_cost);
        println!("   {} - {} ({})", card.type_line, card.set_name, card.set);
    }
}

pub fn display_card(card: &Card) {
    let card_header = if let Some(mana_cost) = &card.mana_cost {
        format!("{} {}", card.name, mana_cost)
    } else {
        card.name.clone()
    };

    println!("\n════════════════════════════════════════");
    println!("{}", card_header);
    println!("════════════════════════════════════════");
    
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
