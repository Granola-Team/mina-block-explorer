use crate::{
    common::{constants::*, models::*},
    icons::*,
};
use leptos::*;
use serde::{Deserialize, Serialize};

enum Icon {
    Docs,
    Terms,
    Support,
}
pub struct Link<'a> {
    label: &'a str,
    href: &'a str,
    icon: Icon,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
struct VersionData {
    pub version: String,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
struct APIVersionResponse {
    pub data: VersionData,
}

async fn load_api_version_data() -> Result<APIVersionResponse, MyError> {
    let query_body = r#"{"query":"query VersionQuery { version }","operationName":"VersionQuery"}"#;
    let client = reqwest::Client::new();
    let response = client
        .post(GRAPHQL_ENDPOINT)
        .body(query_body)
        .send()
        .await
        .map_err(|e| MyError::NetworkError(e.to_string()))?;

    if response.status().is_success() {
        let summary = response
            .json::<APIVersionResponse>()
            .await
            .map_err(|e| MyError::ParseError(e.to_string()))?;
        Ok(summary)
    } else {
        Err(MyError::NetworkError("Failed to fetch data".into()))
    }
}

const FOOTER_LINK_BASE_CLASS: &str = "ml-2 flex items-center text-white text-sm ";
const HIDE_ON_MOBILE: &str = "hidden sm:block ";

#[component]
pub fn Footer() -> impl IntoView {
    let resource = create_resource(|| (), |_| async move { load_api_version_data().await });

    let links = vec![
        Link {
            label: "Docs",
            href: "https://github.com/Granola-Team/mina-block-explorer/blob/main/DOCS/SITE_DOCS.md",
            icon: Icon::Docs,
        },
        Link {
            label: "Terms",
            href: "https://gist.github.com/jhult/0a633e7771a695b0ffa529ab55722523",
            icon: Icon::Terms,
        },
        Link {
            label: "Support",
            href: "https://docs.minaexplorer.com/minaexplorer/get-help",
            icon: Icon::Support,
        },
    ];
    view! {
        <footer class="overflow-x-auto bg-main-background w-full h-14 min-h-14 flex flex-wrap justify-end">
            <div class="w-full flex justify-between sm:justify-end p-4">
                <span class="flex text-white text-sm whitespace-nowrap items-center justify-start grow">
                    <span class="whitespace-pre ".to_string() + HIDE_ON_MOBILE>"Powered by "</span>
                    <a
                        href="https://granola.team"
                        class="flex items-center text-sm".to_string() + LINK_HOVER_STATE
                    >
                        "Granola"
                    </a>
                    <a
                        target="_blank"
                        class=FOOTER_LINK_BASE_CLASS.to_string() + LINK_HOVER_STATE + HIDE_ON_MOBILE
                        href="https://github.com/Granola-Team/mina-block-explorer/commit/"
                            .to_string() + COMMIT_HASH
                    >
                        {&COMMIT_HASH[..7]}
                    </a>
                    {move || match resource.get().and_then(|res| res.ok()) {
                        Some(res) => {
                            let commit = res
                                .data
                                .version
                                .split('-')
                                .collect::<Vec<_>>()[1]
                                .to_string();
                            view! {
                                <span class="ml-2 ".to_string() + HIDE_ON_MOBILE>"|"</span>
                                <a
                                    target="_blank"
                                    class=FOOTER_LINK_BASE_CLASS.to_string() + LINK_HOVER_STATE
                                        + HIDE_ON_MOBILE
                                    href="https://github.com/Granola-Team/mina-indexer/commit/"
                                        .to_string() + &commit
                                >
                                    {res.data.version}
                                </a>
                            }
                        }
                        _ => ().into_view().into(),
                    }}

                </span>
                {links
                    .into_iter()
                    .map(|link| {
                        view! {
                            <a
                                class="uppercase ".to_string() + FOOTER_LINK_BASE_CLASS
                                    + LINK_HOVER_STATE
                                href=link.href
                            >
                                {match link.icon {
                                    Icon::Docs => view! { <DocsIcon width=12/> },
                                    Icon::Terms => view! { <TermsIcon width=12/> },
                                    Icon::Support => view! { <SupportIcon width=12/> },
                                }}

                                <div class="ml-1">{link.label}</div>
                            </a>
                        }
                    })
                    .collect::<Vec<_>>()}
            </div>
        </footer>
    }
}
