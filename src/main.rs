use pony_name_query::data::{magic_cards::Cards, pony_cards::Ponies, *};

fn main() {
    let data_directory = "src/data";

    let pony_cards = "pony_cards.json";
    let magic_cards = "/magic_cards.json";

    let current_directory = std::env::current_dir().unwrap();

    let pony_cards_path = format!(
        "{}/{}/{}",
        current_directory.display().to_string(),
        data_directory,
        pony_cards
    );

    let magic_cards_path = format!(
        "{}/{}/{}",
        current_directory.display().to_string(),
        data_directory,
        magic_cards
    );

    // Parse the JSON.
    // Read pony_cards.json into the Cards data type.
    let pony_cards_json = std::fs::read_to_string(pony_cards_path).unwrap();
    let pony_cards: Ponies = serde_json::from_str(&pony_cards_json).unwrap();

    // Read magic_cards.json into the Cards data type.
    let magic_cards_json = std::fs::read_to_string(magic_cards_path).unwrap();
    let magic_cards: Cards = serde_json::from_str(&magic_cards_json).unwrap();

    let pony_names: Vec<String> = pony_cards
        .iter()
        .map(|card| card.faces.iter().map(|face| face.name.clone()).collect())
        .collect();

    let magic_names: Vec<String> = magic_cards.iter().map(|card| card.name.clone()).collect();

    // Find the names that are in both lists.
    let mut names: Vec<String> = Vec::new();

    for pony_name in &pony_names {
        for magic_name in &magic_names {
            if pony_name == magic_name {
                names.push(pony_name.clone());
            }
        }
    }

    // Print the names.
    for name in names {
        println!("{}", name);
    }
}
