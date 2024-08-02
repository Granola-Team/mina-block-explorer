use super::graphql::{account_activity_query, AccountActivityQuery};
use crate::{
    account_activity::{
        graphql::account_activity_query::{
            AccountActivityQueryAccounts, AccountActivityQueryDelegate,
            AccountActivityQueryDelegators, BlockCreatorAccountQueryInput,
            BlockProtocolStateConsensusStateQueryInput, BlockProtocolStateQueryInput,
            BlockQueryInput,
        },
        models::AccountActivityQueryDelegatorExt,
    },
    common::{constants::*, functions::*, models::*, spotlight::*},
};
use graphql_client::reqwest::post_graphql;

#[allow(clippy::too_many_arguments)]
pub async fn load_data(
    public_key: Option<String>,
    blocks_limit: Option<u64>,
    snarks_limit: Option<u64>,
    trans_limit: Option<u64>,
    delegators_limit: Option<u64>,
    internal_commands_limit: Option<u64>,
    block_height: Option<u64>,
    txn_hash: Option<String>,
    state_hash: Option<String>,
    prover: Option<String>,
    nonce: Option<u64>,
    counterparty: Option<String>,
    slot: Option<u64>,
    block_producer: Option<String>,
    current_epoch_staking_ledger: Option<u64>,
    canonical: Option<bool>,
) -> Result<account_activity_query::ResponseData, MyError> {
    let block_height = block_height.map(|x| x as i64);
    let nonce = nonce.map(|x| x as i64);
    let slot = slot.map(|x| x as i64);
    let get_current_epoch_staking_ledger =
        move || current_epoch_staking_ledger.and_then(|e| e.try_into().ok());

    let variables = account_activity_query::Variables {
        blocks_sort_by: account_activity_query::BlockSortByInput::BLOCKHEIGHT_DESC,
        snarks_sort_by: account_activity_query::SnarkSortByInput::BLOCKHEIGHT_DESC,
        trans_sort_by: account_activity_query::TransactionSortByInput::BLOCKHEIGHT_DESC,
        internal_commands_sort_by: account_activity_query::FeetransferSortByInput::BLOCKHEIGHT_DESC,
        delegators_sort_by: account_activity_query::StakeSortByInput::BALANCE_DESC,
        blocks_limit: blocks_limit.map(|x| x as i64),
        snarks_limit: snarks_limit.map(|x| x as i64),
        trans_limit: trans_limit.map(|x| x as i64),
        internal_commands_limit: internal_commands_limit.map(|x| x as i64),
        delegators_limit: delegators_limit.map(|x| x as i64),
        account_query: account_activity_query::AccountQueryInput {
            public_key: public_key.clone(),
            username: None,
            balance_lte: None,
        },
        blocks_query: account_activity_query::BlockQueryInput {
            block_height_lte: block_height,
            state_hash: state_hash.clone(),
            creator_account: block_producer
                .clone()
                .map(|bp| BlockCreatorAccountQueryInput {
                    public_key: Some(bp),
                    ..Default::default()
                }),
            protocol_state: if slot.is_some() {
                Some(BlockProtocolStateQueryInput {
                    consensus_state: Some(BlockProtocolStateConsensusStateQueryInput {
                        slot_since_genesis_lte: slot,
                        ..Default::default()
                    }),
                    ..Default::default()
                })
            } else {
                None
            },
            canonical: if canonical.is_none() {
                Some(true)
            } else {
                canonical
            },
            ..Default::default()
        },
        snarks_query: account_activity_query::SnarkQueryInput {
            block_height_lte: block_height,
            prover,
            block: if block_producer.is_some() || slot.is_some() || state_hash.is_some() {
                Some(BlockQueryInput {
                    state_hash: state_hash.clone(),
                    creator_account: block_producer.clone().map(|bp| {
                        BlockCreatorAccountQueryInput {
                            public_key: Some(bp),
                            ..Default::default()
                        }
                    }),
                    protocol_state: if slot.is_some() {
                        Some(BlockProtocolStateQueryInput {
                            consensus_state: Some(BlockProtocolStateConsensusStateQueryInput {
                                slot_since_genesis_lte: slot,
                                ..Default::default()
                            }),
                            ..Default::default()
                        })
                    } else {
                        None
                    },
                    ..Default::default()
                })
            } else {
                None
            },
            canonical: if canonical.is_none() {
                Some(true)
            } else {
                canonical
            },
            ..Default::default()
        },
        outgoing_trans_query: account_activity_query::TransactionQueryInput {
            block_height_lte: block_height,
            hash: txn_hash.clone(),
            from: public_key.clone(),
            to: counterparty.clone(),
            nonce,
            canonical: if canonical.is_none() {
                Some(true)
            } else {
                canonical
            },
            ..Default::default()
        },
        incoming_trans_query: account_activity_query::TransactionQueryInput {
            block_height_lte: block_height,
            hash: txn_hash,
            to: public_key.clone(),
            from: counterparty,
            nonce,
            canonical: if canonical.is_none() {
                Some(true)
            } else {
                canonical
            },
            ..Default::default()
        },
        internal_commands_query: account_activity_query::FeetransferQueryInput {
            recipient: public_key.clone(),
            block_height_lte: block_height,
            block_state_hash: state_hash.map(|sh| BlockQueryInput {
                state_hash: Some(sh),
                ..Default::default()
            }),
            canonical: if canonical.is_none() {
                Some(true)
            } else {
                canonical
            },
            ..Default::default()
        },
        delegate_query: account_activity_query::StakeQueryInput {
            public_key: public_key.clone(),
            epoch: get_current_epoch_staking_ledger(),
            ..Default::default()
        },
        delegators_query: account_activity_query::StakeQueryInput {
            delegate: public_key,
            epoch: get_current_epoch_staking_ledger(),
            ..Default::default()
        },
    };

    let client = reqwest::Client::new();

    let response = post_graphql::<AccountActivityQuery, _>(&client, GRAPHQL_ENDPOINT, variables)
        .await
        .map_err(|e| MyError::NetworkError(e.to_string()))?;

    if let Some(errors) = response.errors {
        return Err(MyError::GraphQLError(errors));
    }

    response
        .data
        .ok_or(MyError::GraphQLEmpty("No data available".to_string()))
}

pub fn get_spotlight_loading_data() -> Vec<SpotlightEntry> {
    vec![
        SpotlightEntry {
            label: String::from("Balance"),
            ..Default::default()
        },
        SpotlightEntry {
            label: String::from("Nonce"),
            ..Default::default()
        },
        SpotlightEntry {
            label: String::from("Delegate"),
            ..Default::default()
        },
    ]
}

pub fn get_spotlight_data(
    account: &AccountActivityQueryAccounts,
    total_num_blocks: u64,
) -> Vec<SpotlightEntry> {
    let mut balance_el = decorate_with_mina_tag(
        account
            .balance
            .map(|b| nanomina_to_mina(b.try_into().unwrap()))
            .unwrap_or_default(),
    );
    if let Some(false) = account.is_genesis_account {
        balance_el = convert_array_to_span(vec![
            balance_el,
            convert_to_span("Includes 1 MINA account creation fee".to_string())
                .attr("class", "block text-xs font-light text-slate-400"),
        ])
        .attr("class", "block")
    }

    vec![
        SpotlightEntry {
            label: String::from("Balance"),
            any_el: Some(balance_el),
            ..Default::default()
        },
        SpotlightEntry {
            label: String::from("Nonce"),
            any_el: Some(convert_to_pill(
                account
                    .nonce
                    .map(|b| format_number(b.to_string()))
                    .unwrap_or_default(),
                ColorVariant::Grey,
            )),
            ..Default::default()
        },
        SpotlightEntry {
            label: String::from("Delegate"),
            any_el: Some({
                let account = account
                    .delegate
                    .clone()
                    .map(|b| b.to_string())
                    .unwrap_or_default();
                convert_to_link(account.clone(), format!("/addresses/accounts/{}", account))
            }),
            copiable: true,
        },
        SpotlightEntry {
            label: String::from("Updated Block #"),
            any_el: Some(convert_to_pill(
                format_number(total_num_blocks.to_string()),
                ColorVariant::Grey,
            )),
            copiable: true,
        },
    ]
}

pub fn extend_delegator_info(
    delegator: &AccountActivityQueryDelegators,
    delegate: &AccountActivityQueryDelegate,
) -> AccountActivityQueryDelegatorExt {
    let total_delegated_nanomina = delegate
        .delegation_totals
        .as_ref()
        .and_then(|totals| totals.total_delegated_nanomina);

    let delegated_balance = delegator.balance_nanomina;

    let percent_of_delegation = match (delegator.balance_nanomina, total_delegated_nanomina) {
        (Some(balance), Some(total)) if total != 0 => Some((balance as f64 / total as f64) * 100.0),
        _ => None,
    };

    AccountActivityQueryDelegatorExt {
        username: delegator.username.clone(),
        epoch: delegator.epoch,
        public_key: delegator.public_key.clone(),
        delegated_balance,
        percent_of_delegation,
    }
}

#[cfg(test)]
mod extend_delegator_info_tests {
    use super::*;
    use crate::account_activity::graphql::account_activity_query::AccountActivityQueryDelegateDelegationTotals;

    #[test]
    fn test_extend_delegator_info() {
        let delegator = AccountActivityQueryDelegators {
            username: Some("user1".to_string()),
            balance_nanomina: Some(500),
            epoch: Some(42),
            public_key: Some("pub_key_123".to_string()),
        };

        let delegate_totals = AccountActivityQueryDelegateDelegationTotals {
            total_delegated_nanomina: Some(1000),
            count_delegates: Some(1),
        };

        let delegate = AccountActivityQueryDelegate {
            delegation_totals: Some(delegate_totals),
        };

        let extended_delegator_info = extend_delegator_info(&delegator, &delegate);

        assert_eq!(extended_delegator_info.username, Some("user1".to_string()));
        assert_eq!(extended_delegator_info.epoch, Some(42));
        assert_eq!(
            extended_delegator_info.public_key,
            Some("pub_key_123".to_string())
        );
        assert_eq!(extended_delegator_info.delegated_balance, Some(500));
        assert_eq!(extended_delegator_info.percent_of_delegation, Some(50.0));
    }

    #[test]
    fn test_extend_delegator_info_no_balance() {
        let delegator = AccountActivityQueryDelegators {
            username: Some("user1".to_string()),
            balance_nanomina: None,
            epoch: Some(42),
            public_key: Some("pub_key_123".to_string()),
        };

        let delegate_totals = AccountActivityQueryDelegateDelegationTotals {
            total_delegated_nanomina: Some(1000),
            count_delegates: Some(1),
        };

        let delegate = AccountActivityQueryDelegate {
            delegation_totals: Some(delegate_totals),
        };

        let extended_delegator_info = extend_delegator_info(&delegator, &delegate);

        assert_eq!(extended_delegator_info.username, Some("user1".to_string()));
        assert_eq!(extended_delegator_info.epoch, Some(42));
        assert_eq!(
            extended_delegator_info.public_key,
            Some("pub_key_123".to_string())
        );
        assert_eq!(extended_delegator_info.delegated_balance, None);
        assert_eq!(extended_delegator_info.percent_of_delegation, None);
    }

    #[test]
    fn test_extend_delegator_info_no_total_delegated() {
        let delegator = AccountActivityQueryDelegators {
            username: Some("user1".to_string()),
            balance_nanomina: Some(500),
            epoch: Some(42),
            public_key: Some("pub_key_123".to_string()),
        };

        let delegate_totals = AccountActivityQueryDelegateDelegationTotals {
            total_delegated_nanomina: None,
            count_delegates: Some(1),
        };

        let delegate = AccountActivityQueryDelegate {
            delegation_totals: Some(delegate_totals),
        };

        let extended_delegator_info = extend_delegator_info(&delegator, &delegate);

        assert_eq!(extended_delegator_info.username, Some("user1".to_string()));
        assert_eq!(extended_delegator_info.epoch, Some(42));
        assert_eq!(
            extended_delegator_info.public_key,
            Some("pub_key_123".to_string())
        );
        assert_eq!(extended_delegator_info.delegated_balance, Some(500));
        assert_eq!(extended_delegator_info.percent_of_delegation, None);
    }

    #[test]
    fn test_extend_delegator_info_zero_total_delegated() {
        let delegator = AccountActivityQueryDelegators {
            username: Some("user1".to_string()),
            balance_nanomina: Some(500),
            epoch: Some(42),
            public_key: Some("pub_key_123".to_string()),
        };

        let delegate_totals = AccountActivityQueryDelegateDelegationTotals {
            total_delegated_nanomina: Some(0),
            count_delegates: Some(1),
        };

        let delegate = AccountActivityQueryDelegate {
            delegation_totals: Some(delegate_totals),
        };

        let extended_delegator_info = extend_delegator_info(&delegator, &delegate);

        assert_eq!(extended_delegator_info.username, Some("user1".to_string()));
        assert_eq!(extended_delegator_info.epoch, Some(42));
        assert_eq!(
            extended_delegator_info.public_key,
            Some("pub_key_123".to_string())
        );
        assert_eq!(extended_delegator_info.delegated_balance, Some(500));
        assert_eq!(extended_delegator_info.percent_of_delegation, None);
    }
}
