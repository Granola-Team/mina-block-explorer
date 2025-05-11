use super::graphql::internal_commands_query::InternalCommandsQueryFeetransfers;
use crate::common::{
    constants::LHS_MAX_SPACE_FEES,
    functions::*,
    models::{ColorVariant, HasBlockHeight},
    table::*,
};
use leptos::*;

impl TableData for Vec<Option<InternalCommandsQueryFeetransfers>> {
    fn get_rows(&self) -> Vec<Vec<HtmlElement<html::AnyElement>>> {
        self.iter()
            .map(|opt_internal_command| match opt_internal_command {
                Some(internal_command) => vec![
                    convert_to_span(internal_command.get_height()),
                    convert_to_copy_link(
                        internal_command.get_state_hash(),
                        format!("/blocks/{}", internal_command.get_state_hash()),
                    ),
                    convert_to_linkable_address(
                        &internal_command.get_receipient_username(),
                        &internal_command.get_receipient(),
                    ),
                    decorate_with_mina_tag(internal_command.get_fee()),
                    convert_to_pill(internal_command.get_type(), ColorVariant::Grey),
                    convert_to_title(
                        convert_to_local_timezone_formatted(&internal_command.get_block_datetime()),
                        internal_command.get_block_datetime(),
                    ),
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
    fn get_receipient_username(&self) -> String;
    fn get_fee(&self) -> String;
    fn get_type(&self) -> String;
    fn get_block_datetime(&self) -> String;
}

impl InternalCommandTrait for InternalCommandsQueryFeetransfers {
    fn get_height(&self) -> String {
        self.block_height
            .map_or_else(String::new, |t| format_number(t.to_string()))
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
    fn get_receipient_username(&self) -> String {
        self.recipient_username
            .as_ref()
            .map_or_else(String::new, |t| t.to_string())
    }
    fn get_fee(&self) -> String {
        self.fee
            .map(|i| nanomina_to_mina(i as u64))
            .map(|number| format_number_for_html(&number, LHS_MAX_SPACE_FEES))
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

impl HasBlockHeight for InternalCommandsQueryFeetransfers {
    fn block_height(&self) -> Option<i64> {
        self.block_height
    }
}
