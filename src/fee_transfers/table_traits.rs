use leptos::{
    html::{self},
    HtmlElement,
};

use super::functions::*;
use crate::common::functions::*;

use crate::common::models::*;
use crate::common::table::TableData;
use crate::fee_transfers::graphql::fee_transfers_query::FeeTransfersQueryFeetransfers;

impl TableData for Vec<Option<FeeTransfersQueryFeetransfers>> {
    fn get_columns(&self) -> Vec<String> {
        ["Recipient", "Fee", "Type", "Date"]
            .iter()
            .map(ToString::to_string)
            .collect::<Vec<_>>()
    }

    fn get_rows(&self) -> Vec<Vec<HtmlElement<html::AnyElement>>> {
        self.iter()
            .map(|opt_fee_transfer| match opt_fee_transfer {
                Some(fee_transfer) => vec![
                    convert_to_link(
                        get_receipient(fee_transfer),
                        format!("/addresses/accounts/{}", get_receipient(fee_transfer)),
                    ),
                    wrap_in_pill(
                        decorate_with_currency_tag(get_fee(fee_transfer), "mina".to_string()),
                        PillVariant::Orange,
                    ),
                    convert_to_pill(get_type(fee_transfer), PillVariant::Grey),
                    convert_array_to_span(vec![
                        convert_to_span(print_time_since(&get_date_time(fee_transfer))),
                        convert_to_span(get_date_time(fee_transfer))
                            .attr("class", "block text-xs font-extralight text-slate-400"),
                    ])
                    .attr("class", "block"),
                ],
                None => vec![],
            })
            .collect::<Vec<_>>()
    }
}
