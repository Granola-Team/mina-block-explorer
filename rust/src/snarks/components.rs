use super::{functions::*, graphql::snarks_query::SnarksQuerySnarks};
use crate::{
    account_activity::components::*,
    common::{functions::*, table::*},
};
use leptos::*;

struct SubEntry {
    label: String,
    value: String,
}
