#[derive(Clone)]
pub struct TokenData {
    pub token_id: String,
    pub locked: bool,
    pub owner_pk: String,
    pub _owner_token_id: String,
    pub token_symbol: String,
    pub token_holders_count: usize,
    pub token_balance: usize,
    pub txn_count: usize,
    pub _unlock_percent: usize,
}
