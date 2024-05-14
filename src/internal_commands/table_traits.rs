use super::graphql::internal_commands_query::InternalCommandsQueryFeetransfers;
use crate::common::{functions::*, models::ColorVariant, table::*};
use leptos::*;

impl TableData for Vec<Option<InternalCommandsQueryFeetransfers>> {
    fn get_columns(&self) -> Vec<String> {
        ["Height", "State Hash", "Age", "Recipient", "Fee", "Type"]
            .iter()
            .map(ToString::to_string)
            .collect::<Vec<_>>()
    }

    fn get_exact_search_columns(&self) -> Vec<String> {
        ["Height", "State Hash", "Recipient"]
            .iter()
            .map(ToString::to_string)
            .collect::<Vec<_>>()
    }

    fn get_rows(&self) -> Vec<Vec<HtmlElement<html::AnyElement>>> {
        self.iter()
            .map(|opt_internal_command| match opt_internal_command {
                Some(internal_command) => vec![
                    convert_array_to_span(vec![
                        convert_to_status_bubble(internal_command.canonical, None),
                        convert_to_span(internal_command.get_height()),
                    ]),
                    convert_to_link(
                        internal_command.get_state_hash(),
                        format!("/commands/{}", internal_command.get_state_hash()),
                    ),
                    convert_to_link(
                        internal_command.get_receipient(),
                        format!("/addresses/accounts/{}", internal_command.get_receipient()),
                    ),
                    decorate_with_currency_tag(internal_command.get_fee(), "mina".to_string()),
                    convert_to_pill(internal_command.get_type(), ColorVariant::Grey),
                    convert_array_to_span(vec![
                        convert_to_span(print_time_since(&internal_command.get_block_datetime())),
                        convert_to_span(internal_command.get_block_datetime())
                            .attr("class", "block text-xs font-light text-slate-400"),
                    ])
                    .attr("class", "block"),
                ],
                None => vec![],
            })
            .collect::<Vec<_>>()
    }
}

pub trait InternalCommandTrait {
    fn get_height(&self) -> String;
    fn get_state_hash(&self) -> String;
    fn get_receipient(&self) -> String;
    fn get_fee(&self) -> String;
    fn get_type(&self) -> String;
    fn get_block_datetime(&self) -> String;
}

impl InternalCommandTrait for InternalCommandsQueryFeetransfers {
    fn get_height(&self) -> String {
        self.block_height
            .map_or_else(String::new, |t| t.to_string())
    }
    fn get_state_hash(&self) -> String {
        self.block_state_hash
            .as_ref()
            .and_then(|bsh| bsh.state_hash.as_ref())
            .map_or_else(String::new, |t| t.to_string())
    }
    fn get_receipient(&self) -> String {
        self.recipient
            .as_ref()
            .map_or_else(String::new, |t| t.to_string())
    }
    fn get_fee(&self) -> String {
        self.fee
            .map(|i| nanomina_to_mina(i as u64))
            .unwrap_or_default()
    }
    fn get_type(&self) -> String {
        self.type_
            .as_ref()
            .map_or_else(String::new, |t| t.to_string())
    }
    fn get_block_datetime(&self) -> String {
        self.date_time.map_or_else(String::new, |o| o.to_string())
    }
}
