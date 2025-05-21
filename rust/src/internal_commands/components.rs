use crate::{
    common::{components::*, constants::*, models::*, table::*},
    internal_commands::functions::load_data,
    summary::models::BlockchainSummary,
};
use codee::string::JsonSerdeCodec;
use leptos::*;
use leptos_meta::Title;
use leptos_router::create_query_signal;
use leptos_use::storage::use_local_storage;

#[component]
pub fn InternalCommandsTab() -> impl IntoView {
    let (summary_sig, _, _) =
        use_local_storage::<BlockchainSummary, JsonSerdeCodec>(BLOCKCHAIN_SUMMARY_STORAGE_KEY);
    let (data_sig, set_data) = create_signal(None);
    let (recipient, _) = create_query_signal::<String>("q-recipient");
    let (height_sig, _) = create_query_signal::<u64>(QUERY_PARAM_HEIGHT);
    let (row_limit_sig, _) = create_query_signal::<u64>("row-limit");
    let (state_hash_sig, _) = create_query_signal::<String>("q-state-hash");
    let (canonical_sig, _) = create_query_signal::<bool>("canonical");
    let resource = create_resource(
        move || {
            (
                recipient.get(),
                height_sig.get(),
                state_hash_sig.get(),
                canonical_sig.get(),
                row_limit_sig.get(),
            )
        },
        |(opt_recipient, height, state_hash, canonical, row_limit)| async move {
            load_data(
                row_limit.map(|l| l as i64),
                opt_recipient,
                height,
                state_hash,
                canonical,
            )
            .await
        },
    );
    create_effect(move |_| {
        resource
            .get()
            .and_then(|res| res.ok())
            .map(|data| set_data.set(Some(data.feetransfers)))
    });

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
            column: "Recipient".to_string(),
            search_type: ColumnSearchType::Text,
            width: Some(String::from(TABLE_COL_HASH_WIDTH)),
            ..Default::default()
        },
        TableColumn {
            column: "Fee".to_string(),
            width: Some(String::from(TABLE_COL_NUMERIC_WIDTH)),
            ..Default::default()
        },
        TableColumn {
            column: "Type".to_string(),
            width: Some(String::from(TABLE_COL_SHORT_WIDTH)),
            ..Default::default()
        },
        TableColumn {
            column: "Date".to_string(),
            width: Some(String::from(TABLE_COL_DATE_WIDTH)),
            ..Default::default()
        },
    ];

    view! {
        <Title text="Transactions | Internal Commands" />
        <PageContainer>
            <TableSectionTemplate
                table_columns
                data_sig
                metadata=Signal::derive(move || {
                    let mut available_records = None;
                    if height_sig.get().is_none() && recipient.get().is_none()
                        && state_hash_sig.get().is_none()
                    {
                        available_records = canonical_sig
                            .get()
                            .map(|c| {
                                if c {
                                    summary_sig.get().total_num_canonical_internal_commands
                                } else {
                                    (summary_sig.get().total_num_internal_commands as u64)
                                        .saturating_sub(
                                            summary_sig.get().total_num_canonical_internal_commands,
                                        )
                                }
                            })
                            .or_else(|| Some(
                                summary_sig.get().total_num_canonical_internal_commands,
                            ));
                    }
                    Some(TableMetadata {
                        total_records: u64::try_from(summary_sig.get().total_num_internal_commands)
                            .ok(),
                        available_records,
                        displayed_records: u64::try_from(
                                data_sig.get().map(|d| d.len()).unwrap_or_default(),
                            )
                            .unwrap_or_default(),
                    })
                })

                is_loading=resource.loading()
                section_heading="Internal Commands"
                footer=move || {
                    view! {
                        <NextBlockPage
                            data=data_sig.get().unwrap_or(vec![])
                            row_limit=row_limit_sig.get()
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

        </PageContainer>
    }
}
