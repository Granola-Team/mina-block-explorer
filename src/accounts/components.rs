use crate::{
    accounts::graphql::accounts_query::AccountsQueryAccounts,
    common::{components::*, constants::*, functions::*, models::*},
};
use leptos::*;
use leptos_router::*;

#[component]
pub fn NextAccountsPage(
    data: Vec<Option<AccountsQueryAccounts>>,
    row_limit: Option<u64>,
) -> impl IntoView {
    let (_, set_balance) = create_query_signal_with_options::<String>(
        QUERY_PARAM_BALANCE,
        NavigateOptions {
            scroll: false,
            ..Default::default()
        },
    );

    let mut last_balance = None;
    if let Some(Some(last_row)) = data.last() {
        last_balance = last_row
            .balance
            .as_ref()
            .cloned()
            .map(|b| nanomina_to_mina(b as u64))
            .as_ref()
            .and_then(|b| normalize_number_format(b).ok());
    }
    view! {
        <div class="w-full flex justify-center items-center p-4">
            <Button
                style_variant=ButtonStyleVariant::Tertiary
                text="Load Next"
                on_click=move |_| { set_balance.set(last_balance.clone()) }
                class_str="ml-2"
                disabled=data.len() as u64 != row_limit.unwrap_or(TABLE_ROW_LIMIT)
            />
        </div>
    }
}
