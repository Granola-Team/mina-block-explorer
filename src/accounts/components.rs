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
    let (balance_sig, set_balance) = create_query_signal_with_options::<String>(
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
    let last_balance_clone = last_balance.clone();
    view! {
        <div class="w-full flex justify-center items-center p-4">
            <Button
                style_variant=ButtonStyleVariant::Tertiary
                text="Load Next"
                on_click=move |_| {
                    if let Some(next_balance) = get_next_balance(
                        balance_sig.get(),
                        last_balance.clone(),
                    ) {
                        set_balance.set(Some(next_balance.to_string()));
                    }
                }
                class_str="ml-2"
                disabled=data.len() as u64 != row_limit.unwrap_or(TABLE_ROW_LIMIT)
                    || get_next_balance(balance_sig.get(), last_balance_clone.clone()).is_none()
            />
        </div>
    }
}

fn get_next_balance(current_balance: Option<String>, last_balance: Option<String>) -> Option<f64> {
    let current_balance_opt = current_balance.and_then(|s| s.parse::<f64>().ok());
    let last_balance_opt = last_balance.and_then(|s| s.parse::<f64>().ok());

    match (current_balance_opt, last_balance_opt) {
        (Some(current_balance), Some(last_balance)) => {
            if current_balance > last_balance {
                Some(last_balance)
            } else {
                let adjusted_balance = current_balance - 0.000_000_001;
                Some(if adjusted_balance.is_finite() {
                    adjusted_balance
                } else {
                    current_balance
                })
            }
        }
        (None, Some(last_balance)) => Some(last_balance),
        _ => None,
    }
}

#[cfg(test)]
mod get_next_balance_tests {
    use super::*;

    #[test]
    fn test_get_next_balance_current_greater_than_last() {
        let result = get_next_balance(Some("123.46".to_string()), Some("123.45".to_string()));
        assert_eq!(
            result,
            Some(123.45),
            "Should return last_balance when current_balance > last_balance"
        );
    }

    #[test]
    fn test_get_next_balance_current_less_than_or_equal_last() {
        let result = get_next_balance(Some("123.45".to_string()), Some("123.46".to_string()));
        assert_eq!(
            result,
            Some(123.45 - 0.000_000_001),
            "Should return adjusted current_balance when current_balance <= last_balance"
        );
    }

    #[test]
    fn test_get_next_balance_current_none_last_some() {
        let result = get_next_balance(None, Some("123.45".to_string()));
        assert_eq!(
            result,
            Some(123.45),
            "Should return last_balance when current_balance is None"
        );
    }

    #[test]
    fn test_get_next_balance_both_none() {
        let result = get_next_balance(None, None);
        assert_eq!(result, None, "Should return None when both are None");
    }

    #[test]
    fn test_get_next_balance_current_some_last_none() {
        let result = get_next_balance(Some("123.45".to_string()), None);
        assert_eq!(result, None, "Should return None when last_balance is None");
    }

    #[test]
    fn test_get_next_balance_current_invalid() {
        let result = get_next_balance(Some("abc".to_string()), Some("123.45".to_string()));
        assert_eq!(
            result,
            Some(123.45),
            "Should return last_balance for invalid current_balance"
        );
    }

    #[test]
    fn test_get_next_balance_last_invalid() {
        let result = get_next_balance(Some("123.45".to_string()), Some("abc".to_string()));
        assert_eq!(result, None, "Should return None for invalid last_balance");
    }

    #[test]
    fn test_get_next_balance_both_valid_small_values() {
        let result = get_next_balance(
            Some("0.0000000001".to_string()),
            Some("0.0000000002".to_string()),
        );
        assert_eq!(
            result,
            Some(0.0000000001 - 0.000_000_001),
            "Should return adjusted current_balance for small values"
        );
    }
}
