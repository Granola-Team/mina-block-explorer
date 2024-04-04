use crate::icons::*;
use leptos::*;
mod ga_opt_out;
use ga_opt_out::GAOptOut;

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

#[component]
pub fn Footer() -> impl IntoView {
    let links = vec![
        Link {
            label: "Docs",
            href: "https://docs.minaexplorer.com/minaexplorer",
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
        <footer class="overflow-x-auto bg-main-background w-full h-12 min-h-12 flex flex-wrap justify-end md:grid md:grid-cols-[10%_80%_10%]">
            <div class="md:col-start-2 md:col-end-3 w-full flex justify-between sm:justify-end p-4">
                <GAOptOut/>
                {links
                    .into_iter()
                    .map(|link| {
                        view! {
                            <a
                                class="ml-1 sm:ml-4 flex items-center text-white text-xs uppercase hover:text-granola-orange hover:underline"
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
