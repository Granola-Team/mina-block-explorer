use crate::{
    common::{
        components::*, constants::*, functions::*, models::PageDimensions, search::SearchBar,
        table::*,
    },
    internal_commands::{
        functions::load_data, graphql::internal_commands_query::InternalCommandsQueryFeetransfers,
    },
};
use leptos::*;
use leptos_meta::Title;
use leptos_router::create_query_signal;

#[component]
pub fn InternalCommands(
    internal_commands: Vec<Option<InternalCommandsQueryFeetransfers>>,
) -> impl IntoView {
    view! {
        <SearchBar placeholder=MULTI_SEARCH_PLACEHOLDER_TEXT/>
        <Title text="Transactions | Internal Commands"/>
        <PageContainer>
            <TableSection section_heading="Internal Commands" controls=|| ().into_view()>
                <InternalCommandsTable internal_commands/>
            </TableSection>
        </PageContainer>
    }
}

#[component]
pub fn InternalCommandsTable(
    internal_commands: Vec<Option<InternalCommandsQueryFeetransfers>>,
) -> impl IntoView {
    let page_dim = use_context::<ReadSignal<PageDimensions>>()
        .expect("there to be a `PageDimensions` signal provided");
    let (current_page, set_current_page) = create_signal(1);

    view! {
        {move || {
            let pag = build_pagination(
                internal_commands.len(),
                TABLE_DEFAULT_PAGE_SIZE,
                current_page.get(),
                set_current_page,
                page_dim.get().height.map(|h| h as usize),
                Some(
                    Box::new(|container_height: usize| {
                        (container_height - DEFAULT_ESTIMATED_NON_TABLE_SPACE_IN_SECTIONS)
                            / ESTIMATED_ROW_HEIGHT
                    }),
                ),
            );
            let subset = get_subset(
                &internal_commands,
                pag.records_per_page,
                current_page.get() - 1,
            );
            view! { <Table data=subset pagination=pag/> }
        }}
    }
}

#[component]
pub fn InternalCommandsTab() -> impl IntoView {
    let (recipient, _) = create_query_signal::<String>("query");
    let resource = create_resource(
        move || recipient.get(),
        |opt_recipient| async move { load_data(TABLE_RECORD_SIZE, opt_recipient).await },
    );
    view! {
        {move || match resource.get() {
            Some(Ok(data)) => view! { <InternalCommands internal_commands=data.feetransfers/> },
            Some(Err(_)) => {
                view! {
                    <EmptyTable message="Unable to list internal commands at this time. Try refreshing."/>
                }
            }
            None => view! { <EmptyTable message="No internal commands found"/> },
        }}
    }
}
