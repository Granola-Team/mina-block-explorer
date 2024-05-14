use super::{components::*, functions::*, models::*};
use crate::{
    common::{
        components::*,
        constants::{TABLE_RECORD_SIZE, *},
        functions::*,
        models::{MyError, PageDimensions},
        table::*,
    },
    summary::functions::load_data as load_summary_data,
};
use leptos::*;
use leptos_meta::Title;
use leptos_router::*;

#[component]
pub fn StakesPage() -> impl IntoView {
    let (epoch_sig, _) = create_query_signal::<i64>("epoch");

    view! {
        <Title
            text=move || {
                if let Some(epoch) = epoch_sig.get() {
                    format!("Epoch {}", epoch)
                } else {
                    "Current Staking Ledger".to_string()
                }
            }

            formatter=move |text| format!("Staking Ledger | {text}")
        />
        <PageContainer>
            <StakesPageContents/>
        </PageContainer>
    }
}

#[component]
fn StakesPageContents() -> impl IntoView {
    let (epoch_sig, _) = create_query_signal::<i64>("epoch");
    let query_params_map = use_query_map();

    let summary_resource = create_resource(|| (), |_| async move { load_summary_data().await });

    let current_epoch = move || match summary_resource.get() {
        Some(Ok(data)) => Some(data.epoch as i64),
        _ => None,
    };

    let resource = create_resource(
        move || (epoch_sig.get(), current_epoch(), query_params_map.get()),
        move |(epoch_opt, c_epoch, params_map)| async move {
            match (c_epoch, epoch_opt) {
                (Some(epoch), None) | (_, Some(epoch)) => {
                    let public_key = params_map.get("q-key").cloned();
                    let delegate = params_map.get("q-delegate").cloned();
                    load_data(TABLE_RECORD_SIZE, Some(epoch), public_key, delegate).await
                }
                _ => Err(MyError::ParseError(String::from(
                    "missing epoch information",
                ))),
            }
        },
    );

    let page_dim = use_context::<ReadSignal<PageDimensions>>()
        .expect("there to be a `PageDimensions` signal provided");
    let (current_page, set_current_page) = create_signal(1);

    let get_data_and_pagination = move || {
        resource.get().and_then(|res| res.ok()).map(|data| {
            let pag = build_pagination(
                data.stakes.len(),
                TABLE_DEFAULT_PAGE_SIZE,
                current_page.get(),
                set_current_page,
                page_dim.get().height.map(|h| h as usize),
                Some(Box::new(|container_height: usize| {
                    (container_height - DEFAULT_ESTIMATED_NON_TABLE_SPACE_IN_SECTIONS)
                        / ESTIMATED_ROW_HEIGHT
                })),
            );
            (data, pag)
        })
    };

    let table_columns = vec![
        TableColumn {
            column: "Key".to_string(),
            is_searchable: true,
        },
        TableColumn {
            column: "Stake".to_string(),
            is_searchable: false,
        },
        TableColumn {
            column: "Delegate".to_string(),
            is_searchable: true,
        },
        TableColumn {
            column: "Delegators".to_string(),
            is_searchable: false,
        },
        TableColumn {
            column: "Ledger Hash".to_string(),
            is_searchable: false,
        },
    ];
    let table_cols_length = table_columns.len();
    let table_columns_clone = table_columns.clone();

    let get_heading_and_epochs = create_memo(move |_| {
        summary_resource
            .get()
            .and_then(|res| res.ok())
            .map(|sum_data| {
                let curr_epoch = sum_data.epoch as i64;
                let mut section_heading = "Current Staking Ledger".to_string();
                let mut next_epoch = curr_epoch + 1;
                let mut prev_epoch = curr_epoch - 1;
                if let Some(qs_epoch) = epoch_sig.get() {
                    if qs_epoch != curr_epoch {
                        section_heading = format!("Epoch {} Staking Ledger", qs_epoch);
                        next_epoch = qs_epoch + 1;
                        prev_epoch = qs_epoch - 1;
                    }
                }
                (
                    section_heading,
                    curr_epoch,
                    next_epoch,
                    prev_epoch,
                    sum_data.slot,
                )
            })
            .unwrap_or(("".to_string(), 0, 0, 0, 0))
    });

    {
        move || {
            let table_columns_clone = table_columns_clone.clone();
            let (section_heading, curr_epoch, next_epoch, prev_epoch, slot) =
                get_heading_and_epochs.get();
            view! {
                <TableSection
                    section_heading=section_heading
                    controls=move || {
                        view! {
                            <EpochButton
                                disabled=prev_epoch < 1
                                text="Previous"
                                style_variant=EpochStyleVariant::Secondary
                                epoch_target=prev_epoch
                            />
                            {if next_epoch - 1 == curr_epoch {
                                view! {
                                    <EpochButton
                                        href="/next-stakes"
                                        text="Next"
                                        style_variant=EpochStyleVariant::Primary
                                    />
                                }
                            } else {
                                view! {
                                    <EpochButton
                                        text="Next"
                                        style_variant=EpochStyleVariant::Primary
                                        epoch_target=next_epoch
                                    />
                                }
                            }}
                        }
                    }

                    additional_info=if next_epoch - 1 == curr_epoch {
                        format!(
                            "{:.2}% complete ({}/{} slots filled)",
                            (slot as f64 / EPOCH_SLOTS as f64) * 100.0,
                            slot,
                            EPOCH_SLOTS,
                        )
                    } else {
                        "".to_string()
                    }
                >

                    <TableContainer>
                        <Table>
                            <TableHeader columns=table_columns_clone.clone()/>
                            <Suspense fallback=move || {
                                view! {
                                    <TableRows data=vec![
                                        vec![LoadingPlaceholder; table_cols_length];
                                        10
                                    ]/>
                                }
                            }>
                                {move || {
                                    get_data_and_pagination()
                                        .map(|(data, pag)| {
                                            view! {
                                                <TableRows data=data
                                                    .stakes[pag
                                                        .start_index()..std::cmp::min(
                                                        pag.end_index() + 1,
                                                        pag.total_records,
                                                    )]
                                                    .to_vec()/>
                                            }
                                        })
                                }}

                            </Suspense>
                        </Table>
                        {move || {
                            get_data_and_pagination()
                                .map(|(_, pag)| {
                                    view! { <Pagination pagination=pag/> }
                                })
                        }}

                    </TableContainer>
                </TableSection>
            }
        }
    }
}
