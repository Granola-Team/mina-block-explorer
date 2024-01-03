use chrono::Utc;
use graphql_client::{reqwest::post_graphql, GraphQLQuery};
use leptos::*;

use crate::{
    api_models::MyError,
    table::{Table, TableData},
    table_section::TableSection,
};

use self::snarks_query::SnarksQuerySnarks;

type DateTime = chrono::DateTime<Utc>;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "graphql/schemas/mina-explorer.graphql",
    query_path = "graphql/queries/snarks.graphql",
    response_derives = "Serialize,PartialEq,Debug,Clone"
)]
pub struct SnarksQuery;

impl TableData for Vec<Option<SnarksQuerySnarks>> {
    fn get_columns(&self) -> Vec<String> {
        vec!["Height", "Date", "Prover", "Work Ids", "State Hash", "Fee"]
            .iter()
            .map(ToString::to_string)
            .collect::<Vec<_>>()
    }

    fn get_rows(&self) -> Vec<Vec<String>> {
        self.iter() // Change `into_iter` to `iter` to avoid taking ownership
            .map(|opt_snark| {
                match opt_snark {
                    Some(snark) => vec![
                        snark
                            .block_height
                            .map_or_else(String::new, |o| o.to_string()),
                        snark.date_time.map_or_else(String::new, |o| o.to_string()),
                        snark
                            .prover
                            .as_ref()
                            .map_or_else(String::new, |o| o.to_string()), // Borrowing with as_ref()
                        snark
                            .work_ids
                            .as_ref()
                            .map_or_else(String::new, |work_ids| {
                                work_ids
                                    .iter()
                                    .map(|o| o.map_or_else(String::new, |o1| o1.to_string()))
                                    .collect::<Vec<_>>()
                                    .join(", ")
                            }),
                        snark.block.as_ref().map_or_else(String::new, |o| {
                            o.state_hash
                                .as_ref()
                                .map_or_else(String::new, |o| o.to_string())
                        }),
                        snark.fee.map_or_else(String::new, |o| o.to_string()),
                    ],
                    None => vec![],
                }
            })
            .collect::<Vec<_>>()
    }
}

async fn load_data() -> Result<snarks_query::ResponseData, MyError> {
    let url = "https://graphql.minaexplorer.com";
    let variables = snarks_query::Variables {
        sort_by: snarks_query::SnarkSortByInput::BLOCKHEIGHT_DESC,
        limit: Some(25),
    };

    let client = reqwest::Client::new();

    let response = post_graphql::<SnarksQuery, _>(&client, url, variables)
        .await
        .map_err(|e| MyError::NetworkError(e.to_string()))
        .unwrap();

    if let Some(errors) = response.errors {
        Err(MyError::GraphQLError(errors))
    } else {
        Ok(response.data.unwrap())
    }
}

#[component]
pub fn SnarksPage() -> impl IntoView {
    let resource = create_resource(|| (), |_| async move { load_data().await });

    view! {
        {move || match resource.get() {
            Some(Ok(data)) => view! {
                <TableSection section_heading="SNARKs".to_owned()>
                    <Table data=data.snarks/>
                </TableSection>
            },
            _ => view! { <span /> }.into_view()
        }}
    }
}
