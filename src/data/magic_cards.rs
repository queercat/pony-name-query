use serde_derive::Deserialize;
use serde_derive::Serialize;

pub type Cards = Vec<Card>;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Card {
    pub object: String,
    pub id: String,
    #[serde(rename = "oracle_id")]
    pub oracle_id: String,
    #[serde(rename = "multiverse_ids")]
    pub multiverse_ids: Vec<i64>,
    #[serde(rename = "mtgo_id")]
    pub mtgo_id: Option<i64>,
    #[serde(rename = "mtgo_foil_id")]
    pub mtgo_foil_id: Option<i64>,
    #[serde(rename = "tcgplayer_id")]
    pub tcgplayer_id: Option<i64>,
    #[serde(rename = "cardmarket_id")]
    pub cardmarket_id: Option<i64>,
    pub name: String,
    pub lang: String,
    #[serde(rename = "released_at")]
    pub released_at: String,
    pub uri: String,
    #[serde(rename = "scryfall_uri")]
    pub scryfall_uri: String,
    pub layout: String,
    #[serde(rename = "highres_image")]
    pub highres_image: bool,
    #[serde(rename = "image_status")]
    pub image_status: String,
    #[serde(rename = "image_uris")]
    pub image_uris: Option<ImageUris>,
    #[serde(rename = "mana_cost")]
    pub mana_cost: Option<String>,
    pub cmc: f64,
    #[serde(rename = "type_line")]
    pub type_line: String,
    #[serde(rename = "oracle_text")]
    pub oracle_text: Option<String>,
    #[serde(default)]
    pub colors: Vec<String>,
    #[serde(rename = "color_identity")]
    pub color_identity: Vec<String>,
    pub keywords: Vec<String>,
    pub legalities: Legalities,
    pub games: Vec<String>,
    pub reserved: bool,
    pub foil: bool,
    pub nonfoil: bool,
    pub finishes: Vec<String>,
    pub oversized: bool,
    pub promo: bool,
    pub reprint: bool,
    pub variation: bool,
    #[serde(rename = "set_id")]
    pub set_id: String,
    pub set: String,
    #[serde(rename = "set_name")]
    pub set_name: String,
    #[serde(rename = "set_type")]
    pub set_type: String,
    #[serde(rename = "set_uri")]
    pub set_uri: String,
    #[serde(rename = "set_search_uri")]
    pub set_search_uri: String,
    #[serde(rename = "scryfall_set_uri")]
    pub scryfall_set_uri: String,
    #[serde(rename = "rulings_uri")]
    pub rulings_uri: String,
    #[serde(rename = "prints_search_uri")]
    pub prints_search_uri: String,
    #[serde(rename = "collector_number")]
    pub collector_number: String,
    pub digital: bool,
    pub rarity: String,
    #[serde(rename = "flavor_text")]
    pub flavor_text: Option<String>,
    #[serde(rename = "card_back_id")]
    pub card_back_id: Option<String>,
    pub artist: String,
    #[serde(rename = "artist_ids")]
    #[serde(default)]
    pub artist_ids: Vec<String>,
    #[serde(rename = "illustration_id")]
    pub illustration_id: Option<String>,
    #[serde(rename = "border_color")]
    pub border_color: String,
    pub frame: String,
    #[serde(rename = "full_art")]
    pub full_art: bool,
    pub textless: bool,
    pub booster: bool,
    #[serde(rename = "story_spotlight")]
    pub story_spotlight: bool,
    #[serde(rename = "edhrec_rank")]
    pub edhrec_rank: Option<i64>,
    pub prices: Prices,
    #[serde(rename = "related_uris")]
    pub related_uris: RelatedUris,
    #[serde(rename = "purchase_uris")]
    pub purchase_uris: Option<PurchaseUris>,
    #[serde(rename = "security_stamp")]
    pub security_stamp: Option<String>,
    pub preview: Option<Preview>,
    pub power: Option<String>,
    pub toughness: Option<String>,
    #[serde(rename = "penny_rank")]
    pub penny_rank: Option<i64>,
    #[serde(rename = "arena_id")]
    pub arena_id: Option<i64>,
    #[serde(rename = "all_parts")]
    #[serde(default)]
    pub all_parts: Vec<AllPart>,
    #[serde(rename = "frame_effects")]
    #[serde(default)]
    pub frame_effects: Vec<String>,
    pub watermark: Option<String>,
    #[serde(rename = "produced_mana")]
    #[serde(default)]
    pub produced_mana: Vec<String>,
    #[serde(rename = "card_faces")]
    #[serde(default)]
    pub card_faces: Vec<CardFace>,
    #[serde(rename = "tcgplayer_etched_id")]
    pub tcgplayer_etched_id: Option<i64>,
    #[serde(rename = "promo_types")]
    #[serde(default)]
    pub promo_types: Vec<String>,
    pub loyalty: Option<String>,
    #[serde(rename = "life_modifier")]
    pub life_modifier: Option<String>,
    #[serde(rename = "hand_modifier")]
    pub hand_modifier: Option<String>,
    #[serde(rename = "attraction_lights")]
    #[serde(default)]
    pub attraction_lights: Vec<i64>,
    #[serde(rename = "color_indicator")]
    #[serde(default)]
    pub color_indicator: Vec<String>,
    #[serde(rename = "content_warning")]
    pub content_warning: Option<bool>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ImageUris {
    pub small: String,
    pub normal: String,
    pub large: String,
    pub png: String,
    #[serde(rename = "art_crop")]
    pub art_crop: String,
    #[serde(rename = "border_crop")]
    pub border_crop: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Legalities {
    pub standard: String,
    pub future: String,
    pub historic: String,
    pub gladiator: String,
    pub pioneer: String,
    pub explorer: String,
    pub modern: String,
    pub legacy: String,
    pub pauper: String,
    pub vintage: String,
    pub penny: String,
    pub commander: String,
    pub oathbreaker: String,
    pub brawl: String,
    pub historicbrawl: String,
    pub alchemy: String,
    pub paupercommander: String,
    pub duel: String,
    pub oldschool: String,
    pub premodern: String,
    pub predh: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Prices {
    pub usd: Option<String>,
    #[serde(rename = "usd_foil")]
    pub usd_foil: Option<String>,
    #[serde(rename = "usd_etched")]
    pub usd_etched: Option<String>,
    pub eur: Option<String>,
    #[serde(rename = "eur_foil")]
    pub eur_foil: Option<String>,
    pub tix: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RelatedUris {
    pub gatherer: Option<String>,
    #[serde(rename = "tcgplayer_infinite_articles")]
    pub tcgplayer_infinite_articles: Option<String>,
    #[serde(rename = "tcgplayer_infinite_decks")]
    pub tcgplayer_infinite_decks: Option<String>,
    pub edhrec: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PurchaseUris {
    pub tcgplayer: String,
    pub cardmarket: String,
    pub cardhoarder: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Preview {
    pub source: String,
    #[serde(rename = "source_uri")]
    pub source_uri: String,
    #[serde(rename = "previewed_at")]
    pub previewed_at: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AllPart {
    pub object: String,
    pub id: String,
    pub component: String,
    pub name: String,
    #[serde(rename = "type_line")]
    pub type_line: String,
    pub uri: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CardFace {
    pub object: String,
    pub name: String,
    #[serde(rename = "mana_cost")]
    pub mana_cost: String,
    #[serde(rename = "type_line")]
    pub type_line: Option<String>,
    #[serde(rename = "oracle_text")]
    pub oracle_text: String,
    #[serde(default)]
    pub colors: Vec<String>,
    pub artist: Option<String>,
    #[serde(rename = "artist_id")]
    pub artist_id: Option<String>,
    #[serde(rename = "illustration_id")]
    pub illustration_id: Option<String>,
    #[serde(rename = "image_uris")]
    pub image_uris: Option<ImageUris2>,
    pub power: Option<String>,
    pub toughness: Option<String>,
    #[serde(rename = "flavor_text")]
    pub flavor_text: Option<String>,
    pub defense: Option<String>,
    pub watermark: Option<String>,
    pub loyalty: Option<String>,
    #[serde(rename = "flavor_name")]
    pub flavor_name: Option<String>,
    #[serde(rename = "color_indicator")]
    #[serde(default)]
    pub color_indicator: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ImageUris2 {
    pub small: String,
    pub normal: String,
    pub large: String,
    pub png: String,
    #[serde(rename = "art_crop")]
    pub art_crop: String,
    #[serde(rename = "border_crop")]
    pub border_crop: String,
}
