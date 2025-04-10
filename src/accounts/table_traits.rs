use crate::{
    accounts::graphql::accounts_query::AccountsQueryAccounts,
    common::{constants::LHS_MAX_DIGIT_PADDING, functions::*, models::*, table::TableData},
};
use leptos::*;

impl TableData for Vec<Option<AccountsQueryAccounts>> {
    fn get_rows(&self) -> Vec<Vec<HtmlElement<html::AnyElement>>> {
        self.iter()
            .map(|acct_opt| match acct_opt {
                Some(account) => vec![
                    if account.is_zk_app() {
                        convert_to_pill("Zkapp".to_string(), ColorVariant::Orange)
                    } else {
                        convert_to_span("".to_string())
                    },
                    convert_to_copy_link(
                        account.get_public_key(),
                        format!("/addresses/accounts/{}", account.get_public_key()),
                    ),
                    convert_to_span(account.get_username()),
                    convert_to_span(account.get_balance()),
                    convert_to_pill(account.get_nonce(), ColorVariant::Grey),
                    convert_to_copy_link(
                        account.get_delegate(),
                        format!("/addresses/accounts/{}", account.get_delegate()),
                    ),
                    convert_to_span(account.get_timelocked()),
                ],
                None => vec![],
            })
            .collect::<Vec<_>>()
    }
}

pub trait AccountTrait {
    fn is_zk_app(&self) -> bool;
    fn get_public_key(&self) -> String;
    fn get_username(&self) -> String;
    fn get_balance(&self) -> String;
    fn get_nonce(&self) -> String;
    fn get_delegate(&self) -> String;
    fn get_timelocked(&self) -> String;
}

impl AccountTrait for AccountsQueryAccounts {
    fn is_zk_app(&self) -> bool {
        self.zkapp.is_some()
    }
    fn get_public_key(&self) -> String {
        self.public_key
            .as_ref()
            .cloned()
            .unwrap_or_default()
            .to_string()
    }
    fn get_username(&self) -> String {
        self.username
            .as_ref()
            .cloned()
            .unwrap_or_default()
            .to_string()
    }
    fn get_balance(&self) -> String {
        self.balance
            .as_ref()
            .cloned()
            .map(|b| nanomina_to_mina(b as u64))
            .map(|number| format_number_for_html(&number, LHS_MAX_DIGIT_PADDING))
            .unwrap_or_default()
            .to_string()
    }
    fn get_nonce(&self) -> String {
        format_number(self.nonce.unwrap_or_default().to_string())
    }
    fn get_delegate(&self) -> String {
        self.delegate.as_ref().cloned().unwrap_or_default()
    }
    fn get_timelocked(&self) -> String {
        self.time_locked
            .and_then(|tl| tl.then(|| "yes".to_string()).or(Some("no".to_string())))
            .unwrap()
    }
}
