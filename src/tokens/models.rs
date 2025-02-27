use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct TokenData {
    pub name: String,
    pub id: String,
    pub supply: f64,
    pub owner: String,
    pub holders: i32,
    pub transactions: i32,
    pub locked: bool,
}

#[derive(Default)]
pub enum TokenDataSortBy {
    #[default]
    Name,
    //Supply,
    // Holders,
    // Transactions,
}

impl TokenDataSortBy {
    pub fn as_str(&self) -> &'static str {
        match self {
            TokenDataSortBy::Name => "name",
            // TokenDataSortBy::Supply => "supply",
            // TokenDataSortBy::Holders => "holders",
            // TokenDataSortBy::Transactions => "transactions",
        }
    }
}
