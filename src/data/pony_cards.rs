use serde_derive::Deserialize;
use serde_derive::Serialize;

pub type Ponies = Vec<Pony>;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Pony {
    pub aliases: Vec<String>,
    pub faces: Vec<Face>,
    pub formats: Formats,
    pub id: i64,
    pub number: String,
    pub rarity: String,
    pub set: Set,
    pub version: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Face {
    pub abilities: Vec<String>,
    #[serde(default)]
    pub colors: Vec<String>,
    pub cost: Option<i64>,
    pub flavor: Option<String>,
    pub image: String,
    pub name: String,
    pub orientation: String,
    #[serde(rename = "play_requirements")]
    pub play_requirements: Option<PlayRequirements>,
    pub power: Option<i64>,
    pub subtitle: Option<String>,
    pub title: String,
    pub traits: Vec<String>,
    #[serde(rename = "type")]
    pub type_field: String,
    #[serde(rename = "home_limit")]
    pub home_limit: Option<i64>,
    pub side: Option<String>,
    pub points: Option<i64>,
    #[serde(rename = "bonus_points")]
    pub bonus_points: Option<i64>,
    #[serde(rename = "opponents_requirements")]
    pub opponents_requirements: Option<OpponentsRequirements>,
    #[serde(rename = "your_requirements")]
    pub your_requirements: Option<YourRequirements>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PlayRequirements {
    pub required: Option<Required>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Required {
    #[serde(rename = "Orange")]
    pub orange: Option<i64>,
    #[serde(rename = "White")]
    pub white: Option<i64>,
    #[serde(rename = "Blue")]
    pub blue: Option<i64>,
    #[serde(rename = "Yellow")]
    pub yellow: Option<i64>,
    #[serde(rename = "Pink")]
    pub pink: Option<i64>,
    #[serde(rename = "Purple")]
    pub purple: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OpponentsRequirements {
    pub required: Required2,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Required2 {
    #[serde(rename = "Wild")]
    pub wild: Option<i64>,
    #[serde(rename = "Blue")]
    pub blue: Option<i64>,
    #[serde(rename = "Orange")]
    pub orange: Option<i64>,
    #[serde(rename = "Pink")]
    pub pink: Option<i64>,
    #[serde(rename = "Purple")]
    pub purple: Option<i64>,
    #[serde(rename = "White")]
    pub white: Option<i64>,
    #[serde(rename = "Yellow")]
    pub yellow: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct YourRequirements {
    pub prohibited: Option<Prohibited>,
    pub required: Required3,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Prohibited {
    #[serde(rename = "Pink")]
    pub pink: Option<i64>,
    #[serde(rename = "Orange")]
    pub orange: Option<i64>,
    #[serde(rename = "White")]
    pub white: Option<i64>,
    #[serde(rename = "Yellow")]
    pub yellow: Option<i64>,
    #[serde(rename = "Blue")]
    pub blue: Option<i64>,
    #[serde(rename = "Purple")]
    pub purple: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Required3 {
    #[serde(rename = "Pink")]
    pub pink: Option<i64>,
    #[serde(rename = "Wild")]
    pub wild: Option<i64>,
    #[serde(rename = "Orange")]
    pub orange: Option<i64>,
    #[serde(rename = "Yellow")]
    pub yellow: Option<i64>,
    #[serde(rename = "Blue")]
    pub blue: Option<i64>,
    #[serde(rename = "White")]
    pub white: Option<i64>,
    #[serde(rename = "Purple")]
    pub purple: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Formats {
    #[serde(rename = "Adventure")]
    pub adventure: String,
    #[serde(rename = "Core")]
    pub core: String,
    #[serde(rename = "DE Block")]
    pub de_block: String,
    #[serde(rename = "EO Block")]
    pub eo_block: String,
    #[serde(rename = "Harmony")]
    pub harmony: String,
    #[serde(rename = "LL Block")]
    pub ll_block: String,
    #[serde(rename = "PR Block")]
    pub pr_block: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Set {
    pub code: String,
    pub name: String,
}
