use crate::{
    common::{components::*, constants::*, functions::*, models::PageDimensions, table::*},
    internal_commands::functions::load_data,
};
use leptos::*;
use leptos_meta::Title;
use leptos_router::create_query_signal;

#[component]
pub fn InternalCommandsTab() -> impl IntoView {
    view! {
        <Title text="Transactions | Internal Commands"/>
        <PageContainer>
            <TableSection section_heading="Internal Commands" controls=|| ().into_view()>
                <InternalCommandsTable/>
            </TableSection>
        </PageContainer>
    }
}

#[component]
pub fn InternalCommandsTable() -> impl IntoView {
    let (recipient, _) = create_query_signal::<String>("q-recipient");
    let resource = create_resource(
        move || recipient.get(),
        |opt_recipient| async move { load_data(TABLE_RECORD_SIZE, opt_recipient).await },
    );
    let page_dim = use_context::<ReadSignal<PageDimensions>>()
        .expect("there to be a `PageDimensions` signal provided");
    let (current_page, set_current_page) = create_signal(1);

    let table_columns = vec![
        TableColumn {
            column: "Recipient".to_string(),
            is_searchable: true,
        },
        TableColumn {
            column: "Fee".to_string(),
            is_searchable: false,
        },
        TableColumn {
            column: "Type".to_string(),
            is_searchable: false,
        },
        TableColumn {
            column: "Age".to_string(),
            is_searchable: false,
        },
    ];
    let table_cols_length = table_columns.len();
    let get_data_and_pagination = move || {
        resource.get().and_then(|res| res.ok()).map(|data| {
            let pag = build_pagination(
                data.feetransfers.len(),
                TABLE_DEFAULT_PAGE_SIZE,
                current_page.get(),
                set_current_page,
                page_dim.get().height.map(|h| h as usize),
                Some(Box::new(|container_height: usize| {
                    (container_height - DEFAULT_ESTIMATED_NON_TABLE_SPACE_IN_SECTIONS)
                        / ESTIMATED_ROW_HEIGHT
                })),
            );
            (data, pag)
        })
    };

    view! {
        <TableContainer>
            <Table>
                <TableHeader columns=table_columns/>
                <Suspense fallback=move || {
                    view! {
                        <TableRows data=vec![vec![LoadingPlaceholder; table_cols_length]; 10]/>
                    }
                }>
                    {move || {
                        get_data_and_pagination()
                            .map(|(data, pag)| {
                                if data.feetransfers.is_empty() {
                                    return view! {
                                        <EmptyTable message="No internal commands found"/>
                                    };
                                }
                                view! {
                                    <TableRows data=data
                                        .feetransfers[pag.start_index()..pag.end_index()]
                                        .to_vec()/>
                                }
                            })
                    }}

                </Suspense>
            </Table>
            {move || {
                get_data_and_pagination()
                    .map(|(_, pag)| {
                        view! { <Pagination pagination=pag/> }
                    })
            }}

        </TableContainer>
    }
}
