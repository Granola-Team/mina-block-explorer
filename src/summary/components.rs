use crate::{
    common::{
        components::AppSection,
        constants::EPOCH_SLOTS,
        functions::{convert_to_pill, data_placeholder},
        models::*,
    },
    summary::functions::*,
};
use leptos::*;
#[component]
pub fn EpochSlotIndicator() -> impl IntoView {
    let blockchain_summary_resource = create_resource(|| (), |_| async move { load_data().await });

    let class_base = "font-light text-sm p-4 whitespace-nowrap flex w-fit";

    view! {
        <AppSection>
            <div class="w-full flex justify-end items-end md:items-center flex-col md:flex-row flex-row-reverse md:flex-row pg-container">
                <Suspense fallback=move || {
                    view! {
                        <span class=class_base>{data_placeholder()}</span>
                        <span class=class_base>{data_placeholder()}</span>
                        <span class=class_base>{data_placeholder()}</span>
                    }
                }>
                    {move || {
                        blockchain_summary_resource
                            .get()
                            .map(|res| {
                                match res {
                                    Ok(data) => {
                                        {
                                            let percent_complete = (data.slot as f64
                                                / EPOCH_SLOTS as f64) * 100.0;
                                            view! {
                                                <span class=class_base.to_string()
                                                    + " pg-completeness">
                                                    {format!("Epoch is {:.2}% complete", percent_complete)}
                                                </span>
                                                <span class=class_base.to_string()
                                                    + " pg-slot">
                                                    {convert_to_pill(
                                                        format!("Current slot: {}", data.slot),
                                                        ColorVariant::Grey,
                                                    )}

                                                </span>
                                                <span class=class_base.to_string()
                                                    + " pg-total-slots">
                                                    {convert_to_pill(
                                                        format!("Epoch slots: {}", EPOCH_SLOTS),
                                                        ColorVariant::Grey,
                                                    )}

                                                </span>
                                            }
                                        }
                                            .into_view()
                                    }
                                    _ => ().into_view(),
                                }
                            })
                    }}

                </Suspense>
            </div>
        </AppSection>
    }
}
