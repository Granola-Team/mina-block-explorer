use super::models::AllAccountSummary;
use crate::common::{functions::*, models::*, table::TableData};
use leptos::*;

impl TableData for Vec<Option<AllAccountSummary>> {
    fn get_columns(&self) -> Vec<String> {
        vec![
            String::from("Public Key"),
            String::from("Username"),
            String::from("Balance"),
            String::from("Nonce"),
            String::from("Delegate"),
            String::from("Time Locked"),
        ]
    }

    fn get_rows(&self) -> Vec<Vec<HtmlElement<html::AnyElement>>> {
        self.iter()
            .map(|all_acct_sum_opt| match all_acct_sum_opt {
                Some(all_account_sum) => vec![
                    convert_to_link(
                        all_account_sum.pk.to_string(),
                        "/addresses/accounts/B62qr9zQ1LKnKM3d7wmFVjv3TzSV6fnXBZ162scK49NTHCA8Xc7PVKq".to_string(),
                    ),
                    convert_to_span(all_account_sum.username.to_string()),
                        decorate_with_mina_tag(
                            all_account_sum.balance.to_string()
                        ),
                    convert_to_pill(all_account_sum.nonce.to_string(), ColorVariant::Grey),
                    convert_to_link(
                        all_account_sum.delegate.to_string(),
                        "/addresses/accounts/B62qr9zQ1LKnKM3d7wmFVjv3TzSV6fnXBZ162scK49NTHCA8Xc7PVKq".to_string(),
                    ),
                    convert_to_span(false.to_string()),
                ],
                None => vec![],
            })
            .collect::<Vec<_>>()
    }
}
