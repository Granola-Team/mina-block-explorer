use crate::{common::constants::*, icons::*, summary::models::*};
use codee::string::JsonSerdeCodec;
use leptos::*;
use leptos_use::storage::use_local_storage;
use serde::{Deserialize, Serialize};

enum Icon {
    Docs,
    Api,
    Discord,
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

const FOOTER_LINK_BASE_CLASS: &str = "ml-4 flex items-center text-white text-sm ";
const HIDE_ON_MOBILE: &str = "hidden sm:block ";

#[component]
pub fn Footer() -> impl IntoView {
    let (summary_sig, _, _) =
        use_local_storage::<BlockchainSummary, JsonSerdeCodec>(BLOCKCHAIN_SUMMARY_STORAGE_KEY);

    let links = vec![
        Link {
            label: "Docs",
            href: "https://granola.gitbook.io/minasearch",
            icon: Icon::Docs,
        },
        Link {
            label: "API",
            href: "https://granola.gitbook.io/minasearch/apis",
            icon: Icon::Api,
        },
        Link {
            label: "Discord",
            href: "https://discord.gg/Zvu6XHNCxj",
            icon: Icon::Discord,
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
                        {COMMIT_HASH}
                    </a>

                    {move || {
                        let version = summary_sig.get().clone().indexer_version.clone();
                        let commits = version.split('-').map(|s| s.to_owned()).collect::<Vec<_>>();
                        commits
                            .last()
                            .cloned()
                            .map(|commit| {
                                view! {
                                    // Ensure each split result is owned
                                    <span class="ml-2 ".to_string() + HIDE_ON_MOBILE>{"|"}</span>
                                    <a
                                        target="_blank"
                                        class=FOOTER_LINK_BASE_CLASS.to_string() + LINK_HOVER_STATE
                                            + HIDE_ON_MOBILE
                                        href="https://github.com/Granola-Team/mina-indexer/commit/"
                                            .to_string() + &commit
                                    >
                                        {commit}
                                    </a>
                                }
                            })
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
                                    Icon::Docs => view! { <DocsIcon width=16 /> },
                                    Icon::Api => view! { <ApiIcon width=16 /> },
                                    Icon::Discord => view! { <DiscordIcon width=16 /> },
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
