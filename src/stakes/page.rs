use super::components::*;
use crate::{
    common::{components::*, constants::*, models::MyError},
    summary::models::BlockchainSummary,
};
use codee::string::JsonSerdeCodec;
use leptos::*;
use leptos_meta::Title;
use leptos_router::*;
use leptos_use::storage::use_local_storage;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Default)]
struct EpochSummary {
    pub epoch: u64,
    pub epoch_num_accounts: u64,
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Default)]
struct StakesData {
    pub stakes: Vec<EpochSummary>,
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Default)]
struct EpochSummaryResponse {
    pub data: StakesData,
}

async fn load_epoch_summary(epoch: Option<u64>) -> Result<EpochSummaryResponse, MyError> {
    match epoch {
        None => Err(MyError::ParseError("epoch not supplied".to_string())),
        Some(epoch) => {
            let query_body = format!(
                r#"{{"query":"query StakingLedgersQuery($limit: Int = 1, $query: StakesQueryInput!) {{stakes(limit: $limit, query: $query) {{ epoch epoch_num_accounts  }}}}", "variables":{{"query": {{"epoch":{}}}, "limit":{}}},"operationName":"StakingLedgersQuery"}}"#,
                epoch, 1
            );
            let client = reqwest::Client::new();
            let response = client
                .post(GRAPHQL_ENDPOINT)
                .body(query_body)
                .send()
                .await
                .map_err(|e| MyError::NetworkError(e.to_string()))?;

            if response.status().is_success() {
                let summary = response
                    .json::<EpochSummaryResponse>()
                    .await
                    .map_err(|e| MyError::ParseError(e.to_string()))?;
                Ok(summary)
            } else {
                Err(MyError::NetworkError("Failed to fetch data".into()))
            }
        }
    }
}

#[component]
pub fn StakesPage() -> impl IntoView {
    let epoch_sig = create_query_signal::<u64>("epoch");
    let (is_berkeley_sig, _) = create_query_signal::<bool>("is-berkeley");
    let (summary_sig, _, _) =
        use_local_storage::<BlockchainSummary, JsonSerdeCodec>(BLOCKCHAIN_SUMMARY_STORAGE_KEY);

    if epoch_sig.0.get_untracked().is_none() {
        epoch_sig.1.set(Some(summary_sig.get().epoch));
    }

    let resource = create_resource(
        move || epoch_sig.0.get(),
        |epoch| async move { load_epoch_summary(epoch).await },
    );

    view! {
        <Title
            text=move || {
                if let Some(epoch) = epoch_sig.0.get() {
                    format!("Epoch {}", epoch)
                } else {
                    "Current".to_string()
                }
            }

            formatter=move |text| format!("Staking Ledger | {text}")
        />
        <PageContainer>
            {move || {
                view! {
                    <StakesPageContents
                        selected_epoch=epoch_sig.0.get()
                        current_epoch=summary_sig.get().epoch
                        slot_in_epoch=summary_sig.get().slot
                        total_num_accounts=Some(summary_sig.get().total_num_accounts)
                        epoch_num_accounts=resource
                            .get()
                            .and_then(|res| res.ok())
                            .and_then(|s| s.data.stakes.first().cloned())
                            .map(|s| s.epoch_num_accounts)
                        genesis_state_hash=is_berkeley_sig
                            .get()
                            .map(|is_berkeley| {
                                if is_berkeley {
                                    HARDFORK_STATE_HASH.to_string()
                                } else {
                                    MAINNET_STATE_HASH.to_string()
                                }
                            })
                    />
                }
            }}

        </PageContainer>
    }
}
