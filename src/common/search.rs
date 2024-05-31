use super::models::MyError;
use crate::{
    common::{components::*, constants::*},
    icons::*,
};
use leptos::*;
use serde::*;

#[derive(Deserialize, Serialize, Debug, Clone)]
struct EpochData {
    pub epoch: i64,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
struct StakeData {
    pub stakes: Vec<EpochData>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
struct EpochConversionResponse {
    pub data: StakeData,
}

async fn load_epoch_data(
    ledger_hash_opt: Option<String>,
) -> Result<EpochConversionResponse, MyError> {
    match ledger_hash_opt {
        None => Err(MyError::ParseError("ledger hash not supplied".to_string())),
        Some(ledger_hash) => {
            let query_body = format!(
                r#"{{"query":"query EpochQuery($ledgerHash: String, $limit: Int) {{ stakes(query: {{ledgerHash: $ledgerHash}}, limit: $limit) {{ epoch }}}}", "variables":{{"ledgerHash":"{}", "limit":{}}},"operationName":"EpochQuery"}}"#,
                ledger_hash, 0
            );
            let client = reqwest::Client::new();
            let response = client
                .post(GRAPHQL_ENDPOINT_2)
                .body(query_body)
                .send()
                .await
                .map_err(|e| MyError::NetworkError(e.to_string()))?;

            if response.status().is_success() {
                let summary = response
                    .json::<EpochConversionResponse>()
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
pub fn GlobalSearchBar() -> impl IntoView {
    let input_element: NodeRef<html::Input> = create_node_ref();
    let (value, set_value) = create_signal("".to_string());
    let (ledger_hash_sig, set_lh) = create_signal(None);
    let epoch_resource = create_resource(
        move || ledger_hash_sig.get(),
        move |_| async move { load_epoch_data(ledger_hash_sig.get()).await },
    );

    let navigate = leptos_router::use_navigate();
    let navigate_clone = navigate.clone();

    create_effect(move |_| {
        epoch_resource.get().and_then(|res| res.ok()).map(|resp| {
            if let Some(stake) = resp.data.stakes.first() {
                navigate_clone(
                    &format!("/staking-ledgers?epoch={}", stake.epoch),
                    Default::default(),
                );
                set_value.set("".to_string());
            }
        })
    });

    let on_submit = move |ev: leptos::ev::SubmitEvent| {
        ev.prevent_default();

        match value.get() {
            val if val.starts_with("B62q") => {
                navigate(&format!("/addresses/accounts/{}", val), Default::default());
                set_value.set("".to_string());
            }
            val if val.starts_with("3N") => {
                navigate(&format!("/blocks/{}", val), Default::default());
                set_value.set("".to_string());
            }
            val if val.starts_with("Ckp") => {
                navigate(&format!("/commands/{}", val), Default::default());
                set_value.set("".to_string());
            }
            val if val.starts_with('j') => {
                set_lh.set(Some(val));
            }
            val if val.chars().all(char::is_numeric) => {
                navigate(
                    &format!("/staking-ledgers?epoch={}", val),
                    Default::default(),
                );
                set_value.set("".to_string());
            }
            _ => {}
        }
    };

    view! {
        <PreSectionContainer>
            <div class="mx-2 my-2 md:mx-0 md:w-full -mt-2 relative align-stretch flex items-center">
                <form class="flex grow" on:submit=on_submit>
                    <input
                        id="searchbar"
                        type="text"
                        on:input=move |ev| {
                            set_value.set(event_target_value(&ev));
                        }

                        prop:value=value
                        placeholder=GLOBAL_SEARCH_PLACEHOLDER_TEXT
                        class="h-14 flex justify-start items-center text-base text-white pl-14 placeholder:text-slate-400 placeholder:font-medium placeholder:text-base focus:outline-none box-border w-full rounded-2xl bg-[#383B42]"
                        node_ref=input_element
                    />
                </form>
                <span class="text-white absolute top-0 left-0 translate-x-3/4 translate-y-3/4">
                    <SearchIcon width=22/>
                </span>
            </div>
        </PreSectionContainer>
    }
}
