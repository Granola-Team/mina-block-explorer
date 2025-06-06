use super::functions::*;
use crate::{
    common::{components::*, constants::*, models::*, table::*},
    snarks::graphql::snarks_query,
    summary::models::BlockchainSummary,
};
use codee::string::JsonSerdeCodec;
use leptos::*;
use leptos_meta::Title;
use leptos_router::{create_query_signal, use_query_map};
use leptos_use::{
    UseIntervalReturn, storage::use_local_storage, use_document_visibility, use_interval,
};
use web_sys::VisibilityState;

#[component]
pub fn SnarksPage() -> impl IntoView {
    view! {
        <Title text="SNARKs | Search For SNARKs" />
        <PageContainer>
            <SnarksPageContents />
        </PageContainer>
    }
}

#[component]
fn SnarksPageContents() -> impl IntoView {
    let (summary_sig, _, _) =
        use_local_storage::<BlockchainSummary, JsonSerdeCodec>(BLOCKCHAIN_SUMMARY_STORAGE_KEY);
    let visibility = use_document_visibility();
    let (data_sig, set_data) = create_signal(None);
    let query_params_map = use_query_map();
    let (canonical_qp, _) = create_query_signal::<bool>("canonical");
    let (block_height_sig, _) = create_query_signal::<u64>(QUERY_PARAM_HEIGHT);
    let (row_limit_sig, _) = create_query_signal::<i64>("row-limit");
    let UseIntervalReturn { counter, .. } = use_interval(LIVE_RELOAD_INTERVAL);

    let resource = create_resource(
        move || {
            (
                counter.get(),
                query_params_map.get(),
                canonical_qp.get(),
                block_height_sig.get(),
                row_limit_sig.get(),
            )
        },
        move |(_, value, canonical, block_height, mut row_limit)| async move {
            if visibility.get_untracked() == VisibilityState::Visible {
                let prover = value.get(QUERY_PARAM_PROVER);
                let block_state_hash = value.get(QUERY_PARAM_STATE_HASH);
                load_data(
                    Some(*row_limit.get_or_insert(25i64)),
                    prover.cloned(),
                    block_state_hash.cloned(),
                    block_height,
                    canonical,
                )
                .await
            } else {
                logging::log!("Document not visible. Data polling skipped snarks query.");
                Ok(snarks_query::ResponseData {
                    snarks: data_sig.get().unwrap_or_default(),
                })
            }
        },
    );

    let table_columns: Vec<TableColumn<AnySort>> = vec![
        TableColumn {
            column: "Height".to_string(),
            search_type: ColumnSearchType::Text,
            html_input_type: "number".to_string(),
            alignment: Some(ColumnTextAlignment::Right),
            width: Some(String::from(TABLE_COL_NUMERIC_WIDTH)),
            ..Default::default()
        },
        TableColumn {
            column: "State Hash".to_string(),
            search_type: ColumnSearchType::Text,
            width: Some(String::from(TABLE_COL_HASH_WIDTH)),
            ..Default::default()
        },
        TableColumn {
            column: "Date".to_string(),
            width: Some(String::from(TABLE_COL_DATE_WIDTH)),
            ..Default::default()
        },
        TableColumn {
            column: "Prover".to_string(),
            search_type: ColumnSearchType::Text,
            width: Some(String::from(TABLE_COL_HASH_WIDTH)),
            ..Default::default()
        },
        TableColumn {
            column: "Fee".to_string(),
            width: Some(String::from(TABLE_COL_NUMERIC_WIDTH)),
            ..Default::default()
        },
    ];

    let get_data = move || resource.get().and_then(|res| res.ok());

    create_effect(move |_| {
        if let Some(data) = get_data() {
            set_data.set(Some(data.snarks))
        }
    });

    view! {
        <TableSectionTemplate
            table_columns
            data_sig
            metadata=Signal::derive(move || {
                let qp_map = query_params_map.get();
                let prover = qp_map.get(QUERY_PARAM_PROVER);
                let block_state_hash = qp_map.get(QUERY_PARAM_STATE_HASH);
                let mut available_records = None;
                if prover.is_none() && block_state_hash.is_none()
                    && block_height_sig.get().is_none()
                {
                    available_records = canonical_qp
                        .get()
                        .map(|c| {
                            if c {
                                summary_sig.get().total_num_canonical_snarks
                            } else {
                                (summary_sig.get().total_num_snarks as u64)
                                    .saturating_sub(summary_sig.get().total_num_canonical_snarks)
                            }
                        })
                        .or_else(|| Some(summary_sig.get().total_num_canonical_snarks));
                }
                Some(TableMetadata {
                    total_records: u64::try_from(summary_sig.get().total_num_snarks).ok(),
                    available_records,
                    displayed_records: u64::try_from(
                            data_sig.get().map(|d| d.len()).unwrap_or_default(),
                        )
                        .unwrap_or_default(),
                    ..Default::default()
                })
            })

            is_loading=resource.loading()
            section_heading="SNARKs"
            footer=move || {
                view! {
                    <NextBlockPage
                        data=data_sig.get().unwrap_or(vec![])
                        row_limit=row_limit_sig.get().map(|l| l as u64)
                    />
                }
            }
            controls=move || {
                view! {
                    <div class="hidden md:flex justify-center items-center">
                        <RowLimit />
                    </div>
                    <UrlParamSelectMenu
                        id="canonical-selection"
                        query_str_key="canonical"
                        labels=UrlParamSelectOptions {
                            is_boolean_option: true,
                            cases: vec!["Canonical".to_string(), "Non-Canonical".to_string()],
                        }
                    />
                }
            }
        />
    }
}
