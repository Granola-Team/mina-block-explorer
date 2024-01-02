use leptos::*;

use crate::icons::*;
use crate::styles::*;

enum Icon {
    Docs,
    Feedback,
    Terms,
    Support
}

#[component]
pub fn Footer() -> impl IntoView {
    let links = vec![
        ("Docs", "https://docs.minaexplorer.com/", Icon::Docs),
        ("Feedback", "https://discord.gg/Sc2JeqCPnX", Icon::Feedback),
        ("Terms", "https://docs.minaexplorer.com/minaexplorer/website-terms-of-service", Icon::Terms),
        ("Support", "https://docs.minaexplorer.com/minaexplorer/get-help", Icon::Support),
    ];
    let breakout_container_styles = get_desktop_breakout_container_styles(None).to_styles_string();
    let breakout_child_styles = get_desktop_breakout_child_styles().to_styles_string();

    view! {
        <footer class={format!("bg-main-background w-full h-12 flex flex-wrap justify-end {}",breakout_container_styles)}>
            <div class={format!("w-full flex justify-between sm:justify-end p-4 sm:px-0 {}",breakout_child_styles)}>    
                {links.into_iter()
                    .map(|(name, link, icon)| view! {
                        <a class="mr-4 flex items-center text-white text-xs uppercase hover:text-granola-orange hover:underline" href=link>
                            {match icon {
                                Icon::Docs => view! { <DocsIcon width=12/> },
                                Icon::Feedback => view! { <FeedbackIcon width=12 /> },
                                Icon::Terms => view! { <TermsIcon width=12 /> },
                                Icon::Support => view! { <SupportIcon width=12 /> },
                            }}
                            <div class="ml-1">{name}</div>
                        </a>
                    }).collect::<Vec<_>>()}
            </div>
        </footer>
    }
}