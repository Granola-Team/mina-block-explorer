#[derive(Clone)]
pub enum BlockContent {
    Spotlight,
    UserCommands,
    FeeTransfers,
    SNARKJobs,
    Analytics,
}

#[derive(Clone, Debug)]
pub struct BlocksQueryBlocksTransactionsUserCommandsExt {
    pub from: Option<String>,
    pub to: Option<String>,
    pub sender_username: Option<String>,
    pub hash: Option<String>,
    pub fee: Option<f64>,
    pub amount: Option<f64>,
    pub kind: Option<String>,
    pub memo: Option<String>,
    pub failure_reason: Option<String>,
    pub nonce: Option<i64>,
    pub block_state_hash: Option<String>,
}
