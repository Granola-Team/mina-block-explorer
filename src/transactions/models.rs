use super::graphql::transactions_query::*;

#[derive(Clone)]
pub struct DirectionalTransactionsQueryTransactions {
    pub base_transaction: TransactionsQueryTransactions,
    pub outbound: bool,
}

impl DirectionalTransactionsQueryTransactions {
    pub fn from_original(
        transactions: &TransactionsQueryTransactions,
        account_owner: String,
    ) -> DirectionalTransactionsQueryTransactions {
        DirectionalTransactionsQueryTransactions {
            base_transaction: transactions.clone(),
            outbound: transactions
                .from
                .clone()
                .map(|a| a == account_owner)
                .unwrap_or(false),
        }
    }
}
