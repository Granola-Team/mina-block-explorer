use crate::{
    accounts::graphql::accounts_query::AccountsQueryAccounts,
    common::{functions::*, models::*, table::TableData},
};
use leptos::*;

impl TableData for Vec<Option<AccountsQueryAccounts>> {
    fn get_rows(&self) -> Vec<Vec<HtmlElement<html::AnyElement>>> {
        self.iter()
            .map(|acct_opt| match acct_opt {
                Some(account) => vec![
                    convert_to_link(
                        account.get_public_key(),
                        format!("/addresses/accounts/{}", account.get_public_key()),
                    ),
                    convert_to_span(account.get_username()),
                    decorate_with_mina_tag(account.get_balance()),
                    convert_to_pill(account.get_nonce(), ColorVariant::Grey),
                    convert_to_link(
                        account.get_delegate(),
                        format!("/addresses/accounts/{}", account.get_delegate()),
                    ),
                    convert_to_span(account.get_timelocked().to_string()),
                ],
                None => vec![],
            })
            .collect::<Vec<_>>()
    }
}

pub trait AccountTrait {
    fn get_public_key(&self) -> String;
    fn get_username(&self) -> String;
    fn get_balance(&self) -> String;
    fn get_nonce(&self) -> String;
    fn get_delegate(&self) -> String;
    fn get_timelocked(&self) -> bool;
}

impl AccountTrait for AccountsQueryAccounts {
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
            .unwrap_or_default()
            .to_string()
    }
    fn get_nonce(&self) -> String {
        self.nonce.unwrap_or_default().to_string()
    }
    fn get_delegate(&self) -> String {
        self.delegate.as_ref().cloned().unwrap_or_default()
    }
    fn get_timelocked(&self) -> bool {
        self.time_locked.unwrap_or(false)
    }
}
