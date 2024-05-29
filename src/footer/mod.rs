use crate::{common::constants::*, icons::*};
use leptos::*;

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

const FOOTER_LINK_BASE_CLASS: &str = "ml-2 flex items-center text-white text-sm ";

#[component]
pub fn Footer() -> impl IntoView {
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
                    <span class="hidden sm:block whitespace-pre">"Powered by "</span>
                    <a
                        href="https://granola.team"
                        class="flex items-center text-sm ".to_string() + LINK_HOVER_STATE
                    >
                        "Granola"
                    </a>
                    <a
                        class=FOOTER_LINK_BASE_CLASS.to_string() + LINK_HOVER_STATE
                        href="https://github.com/Granola-Team/mina-block-explorer/commit/".to_string()
                            + COMMIT_HASH
                    >
                        {&COMMIT_HASH[..7]}
                    </a>
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
