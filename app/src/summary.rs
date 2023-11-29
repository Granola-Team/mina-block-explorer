use leptos::*;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone)]
#[allow(non_snake_case)]
struct BlockchainSummary {
    pub blockchainLength: u64,
    pub circulatingSupply: String,
    pub epoch: u16,
    pub slot: u16,
    pub totalCurrency: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
enum MyError {
    NetworkError(String),
    ParseError(String), // other error variants
}

impl From<reqwest::Error> for MyError {
    fn from(err: reqwest::Error) -> Self {
        MyError::NetworkError(err.to_string())
    }
}

async fn load_data() -> Result<BlockchainSummary, MyError> {
    let response = reqwest::get("https://api.minaexplorer.com/summary")
        .await
        .map_err(|e| MyError::NetworkError(e.to_string()))?;

    if response.status().is_success() {
        let summary = response
            .json::<BlockchainSummary>()
            .await
            .map_err(|e| MyError::ParseError(e.to_string()))?;
        Ok(summary)
    } else {
        Err(MyError::NetworkError("Failed to fetch data".into()))
    }
}

enum ItemValue {
    StrValue(String),
    Int64Value(u64),
    Int16Value(u16),
}

#[component]
fn Item(label: String, value: ItemValue, id: String) -> impl IntoView {
    view! {
        <div class="flex">
            <label for={id.clone()}>{label}:</label>
            <div id={id.clone()}>{
                match value {
                    ItemValue::StrValue(s) => view! {{s}}.into_view(),
                    ItemValue::Int64Value(i) => view! {{i.to_string()}}.into_view(),
                    ItemValue::Int16Value(i) => view! {{i.to_string()}}.into_view()
                }
            }</div>
        </div>
    }
}

#[component]
pub fn Summary() -> impl IntoView {
    let blockchain_summary_resource: Resource<(), Result<BlockchainSummary, MyError>> =
        create_resource(|| (), |_| async move { load_data().await });

    view! {
        <h1>Summary</h1>

            {move || match blockchain_summary_resource.get() {
                None => view! {
                    <div>"Loading..." </div>
                }.into_view(),
                Some(Ok(summary)) => view! {
                    <section class="grid grid-cols-2 gap-1">
                        <Item id="blockchainLength".to_string() label="Height".to_string() value={ItemValue::Int64Value(summary.blockchainLength)} />
                        <Item id="circulatingSupply".to_string() label="Circulating Supply".to_string() value={ItemValue::StrValue(summary.circulatingSupply)} />
                        <Item id="epoch".to_string() label="Epoch".to_string() value={ItemValue::Int16Value(summary.epoch)} />
                        <Item id="slot".to_string() label="Slot".to_string() value={ItemValue::Int16Value(summary.slot)} />
                        <Item id="totalCurrency".to_string() label="Total Currency".to_string() value={ItemValue::StrValue(summary.totalCurrency)} />
                    </section>
                }.into_view(),
                Some(Err(my_error)) => view! {
                    <div> { format!("Error: {:#?}", my_error)}</div>
                }.into_view()
            }}
    }
}
