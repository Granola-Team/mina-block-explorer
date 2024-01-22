
use leptos::{HtmlElement, html::{self}};

use crate::common::functions::*;
use super::functions::*;

use crate::fee_transfers::graphql::fee_transfers_query::FeeTransfersQueryFeetransfers;
use crate::common::table::TableData;
use crate::common::models::*;

impl TableData for &[Option<FeeTransfersQueryFeetransfers>] {
    fn get_columns(&self) -> Vec<String> {
        ["Receipient", "Fee", "Type", "Date"]
            .iter()
            .map(ToString::to_string)
            .collect::<Vec<_>>()
    }

    fn get_rows(&self) -> Vec<Vec<HtmlElement<html::AnyElement>>> {
        self.iter()
            .map(|opt_fee_transfer| match opt_fee_transfer {
                Some(fee_transfer) => vec![
                    convert_to_link(
                        get_receipient(&fee_transfer),
                        format!("/accounts/{}",get_receipient(&fee_transfer))
                    ),
                    convert_to_pill(
                        get_fee(&fee_transfer),
                        PillVariant::Orange
                    ),
                    convert_to_pill(
                        get_type(&fee_transfer),
                        PillVariant::Grey
                    ),
                    convert_to_span(
                        get_date_time(&fee_transfer)
                    ),
                ],
                None => vec![],
            })
            .collect::<Vec<_>>()
    }
}
