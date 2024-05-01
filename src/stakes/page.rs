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
    let (query_sig, _) = create_query_signal::<String>("query");

    let summary_resource = create_resource(|| (), |_| async move { load_summary_data().await });

    let current_epoch = move || match summary_resource.get() {
        Some(Ok(data)) => Some(data.epoch as i64),
        _ => None,
    };

    let resource = create_resource(
        move || (epoch_sig.get(), current_epoch(), query_sig.get()),
        move |(epoch_opt, c_epoch, public_key)| async move {
            match (c_epoch, epoch_opt) {
                (Some(epoch), None) | (_, Some(epoch)) => {
                    load_data(TABLE_RECORD_SIZE, Some(epoch), public_key).await
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
    view! {
        {move || match (resource.get(), summary_resource.get()) {
            (Some(Ok(data)), Some(Ok(sum_data))) => {
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
                let pag = build_pagination(
                    data.stakes.len(),
                    TABLE_DEFAULT_PAGE_SIZE,
                    current_page.get(),
                    set_current_page,
                    page_dim.get().height.map(|h| h as usize),
                    Some(
                        Box::new(|container_height: usize| {
                            (container_height - DEFAULT_ESTIMATED_NON_TABLE_SPACE_IN_SECTIONS)
                                / ESTIMATED_ROW_HEIGHT
                        }),
                    ),
                );
                let subset = get_subset(&data.stakes, pag.records_per_page, current_page.get() - 1);
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
                                (sum_data.slot as f64 / EPOCH_SLOTS as f64) * 100.0,
                                sum_data.slot,
                                EPOCH_SLOTS,
                            )
                        } else {
                            "".to_string()
                        }
                    >

                        <Table data=subset pagination=pag/>
                    </TableSection>
                }
            }
            (_, _) => {
                view! {
                    <TableSection section_heading="" controls=move || ().into_view()>
                        <Table data=LoadingPlaceholder {}/>
                    </TableSection>
                }
            }
        }}
    }
}
