use super::graphql::{account_activity_query, AccountActivityQuery};
use crate::{
    account_activity::graphql::account_activity_query::{
        AccountActivityQueryAccounts, BlockProtocolStateConsensusStateQueryInput,
        BlockProtocolStateQueryInput, BlockQueryInput,
    },
    common::{constants::*, functions::*, models::*, spotlight::*},
};
use graphql_client::reqwest::post_graphql;
use leptos::*;
use leptos_router::*;

pub fn get_base_page_path(location: Location) -> String {
    let path = location.pathname.with(|path| path.clone());
    let path_parts: Vec<&str> = path.split("/accounts").collect();
    match path_parts.first() {
        Some(base) => base.to_string(),
        None => "/".to_string(),
    }
}

#[allow(clippy::too_many_arguments)]
pub async fn load_data(
    public_key: Option<String>,
    blocks_limit: Option<i64>,
    snarks_limit: Option<i64>,
    trans_limit: Option<i64>,
    block_height: Option<i64>,
    txn_hash: Option<String>,
    state_hash: Option<String>,
    prover: Option<String>,
    nonce: Option<i64>,
    counterparty: Option<String>,
    slot: Option<i64>,
    block_producer: Option<String>,
    canonical: Option<bool>,
) -> Result<account_activity_query::ResponseData, MyError> {
    let variables = account_activity_query::Variables {
        blocks_sort_by: account_activity_query::BlockSortByInput::BLOCKHEIGHT_DESC,
        snarks_sort_by: account_activity_query::SnarkSortByInput::BLOCKHEIGHT_DESC,
        trans_sort_by: account_activity_query::TransactionSortByInput::BLOCKHEIGHT_DESC,
        blocks_limit: Some(blocks_limit.unwrap_or_default()),
        snarks_limit: Some(snarks_limit.unwrap_or_default()),
        trans_limit: Some(trans_limit.unwrap_or_default()),
        account_query: account_activity_query::AccountQueryInput {
            public_key: public_key.clone(),
        },
        blocks_query: account_activity_query::BlockQueryInput {
            block_height_lte: block_height,
            state_hash: state_hash.clone(),
            creator: block_producer.clone(),
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
                    state_hash,
                    creator: block_producer,
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
            to: public_key,
            from: counterparty,
            nonce,
            canonical: if canonical.is_none() {
                Some(true)
            } else {
                canonical
            },
            ..Default::default()
        },
    };

    let client = reqwest::Client::new();

    let response = post_graphql::<AccountActivityQuery, _>(&client, GRAPHQL_ENDPOINT_2, variables)
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

pub fn get_spotlight_data(account: &AccountActivityQueryAccounts) -> Vec<SpotlightEntry> {
    vec![
        SpotlightEntry {
            label: String::from("Balance"),
            any_el: Some(decorate_with_mina_tag(
                account
                    .balance
                    .map(|b| nanomina_to_mina(b.try_into().unwrap()))
                    .unwrap_or_default(),
            )),
            ..Default::default()
        },
        SpotlightEntry {
            label: String::from("Nonce"),
            any_el: Some(convert_to_pill(
                account.nonce.map(|b| b.to_string()).unwrap_or_default(),
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
    ]
}
