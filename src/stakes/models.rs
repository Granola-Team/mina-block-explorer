use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct StakesResponse {
    pub data: Vec<Stake>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Stake {
    pub stake: f64,
    pub block_chance: String,
    pub delegates: i32,
    pub name: String,
    pub percent_of_stake: String,
    pub id: Id,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Id {
    pub delegate: String,
}
