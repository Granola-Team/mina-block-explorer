
use chrono::Utc;
use leptos::*;
use graphql_client::{reqwest::post_graphql, GraphQLQuery};

use crate::{table::{TableData, Table}, api_models::MyError, table_section::TableSection};

use self::snarks_query::SnarksQuerySnarks;

type DateTime = chrono::DateTime<Utc>;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "graphql/schemas/mina-explorer.graphql",
    query_path = "graphql/queries/snarks.graphql",
    response_derives = "Serialize,PartialEq,Debug,Clone",
)]
pub struct SnarksQuery;

impl TableData for Vec<Option<SnarksQuerySnarks>> {
    fn get_columns(&self) -> Vec<String> {
        vec![
            String::from("Height"),
            String::from("Date"),
            String::from("Prover"),
            String::from("Work Ids"),
            String::from("State Hash"),
            String::from("Fee"),
        ]
    }

    fn get_rows(&self) -> Vec<Vec<String>> {
        self.into_iter()
            .map(|opt_snark| {
                match opt_snark {
                    Some(snark) => vec![
                        snark.block_height.map_or(String::new(), |o| o.to_string()),
                        snark.date_time.map_or(String::new(), |o| o.to_string()),
                        snark.prover.clone().map_or(String::new(), |o| o.to_string()),
                        snark.work_ids.clone().map_or(String::new(), 
                            |o| o.iter().map(
                                |o1| o1.map_or(String::new(), 
                                    |o2| o2.to_string()))
                            .collect::<Vec<_>>().join(", ")
                        ),
                        snark.block.clone().map_or(String::new(), |o| o.state_hash.map_or(String::new(), |o1| o1.to_string())),
                        snark.fee.map_or(String::new(), |o| o.to_string()),
                    ],
                    None => vec![],
                }
            }).collect::<Vec<_>>()
        
    }
}


async fn load_data() -> Result<snarks_query::ResponseData, MyError> {
    let url = "https://graphql.minaexplorer.com";
    let variables = snarks_query::Variables {
        sort_by: snarks_query::SnarkSortByInput::BLOCKHEIGHT_DESC,
        limit: Some(25)
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