use crate::{
    common::{components::*, constants::*, models::*, table::*},
    internal_commands::functions::load_data,
    summary::models::BlockchainSummary,
};
use leptos::*;
use leptos_meta::Title;
use leptos_router::create_query_signal;
use leptos_use::{storage::use_local_storage, utils::JsonCodec};

#[component]
pub fn InternalCommandsTab() -> impl IntoView {
    let (summary_sig, _, _) =
        use_local_storage::<BlockchainSummary, JsonCodec>(BLOCKCHAIN_SUMMARY_STORAGE_KEY);
    let (data_sig, set_data) = create_signal(None);
    let (recipient, _) = create_query_signal::<String>("q-recipient");
    let (height_sig, _) = create_query_signal::<i64>("q-height");
    let (state_hash_sig, _) = create_query_signal::<String>("q-state-hash");
    let (canonical_sig, _) = create_query_signal::<bool>("canonical");
    let resource = create_resource(
        move || {
            (
                recipient.get(),
                height_sig.get(),
                state_hash_sig.get(),
                canonical_sig.get(),
            )
        },
        |(opt_recipient, height, state_hash, canonical)| async move {
            load_data(opt_recipient, height, state_hash, canonical).await
        },
    );
    create_effect(move |_| {
        resource
            .get()
            .and_then(|res| res.ok())
            .map(|data| set_data.set(Some(data.feetransfers)))
    });

    let table_columns = vec![
        TableColumn {
            column: "Height".to_string(),
            is_searchable: true,
            width: Some(String::from(TABLE_COL_NUMERIC_WIDTH)),
            ..Default::default()
        },
        TableColumn {
            column: "State Hash".to_string(),
            is_searchable: true,
            width: Some(String::from(TABLE_COL_HASH_WIDTH)),
            ..Default::default()
        },
        TableColumn {
            column: "Recipient".to_string(),
            is_searchable: true,
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
            column: "Age".to_string(),
            width: Some(String::from(TABLE_COL_SHORT_WIDTH)),
            ..Default::default()
        },
    ];

    view! {
        <Title text="Transactions | Internal Commands"/>
        <PageContainer>
            <TableSectionTemplate
                table_columns
                data_sig
                metadata=Signal::derive(move || {
                    Some(TableMetadata {
                        total_records: u64::try_from(summary_sig.get().total_num_internal_commands)
                            .ok(),
                        available_records: None,
                        displayed_records: u64::try_from(
                                data_sig.get().map(|d| d.len()).unwrap_or_default(),
                            )
                            .unwrap_or_default(),
                    })
                })

                is_loading=resource.loading()
                section_heading="Internal Commands"
                controls=move || {
                    view! {
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
