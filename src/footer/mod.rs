use leptos::*;

use crate::icons::*;

enum Icon {
    Docs,
    Feedback,
    Terms,
    Support
}
pub struct Link {
    label: String, 
    href: String, 
    icon: Icon
}

#[component]
pub fn Footer() -> impl IntoView {
    let links = vec![
        Link { label: String::from("Docs"), href: String::from("#https://docs.minaexplorer.com/"), icon: Icon::Docs},
        Link { label: String::from("Feedback"), href: String::from("#https://discord.gg/Sc2JeqCPnX"), icon: Icon::Feedback},
        Link { label: String::from("Terms"), href: String::from("#https://docs.minaexplorer.com/minaexplorer/website-terms-of-service"), icon: Icon::Terms},
        Link { label: String::from("Support"), href: String::from("#https://docs.minaexplorer.com/minaexplorer/get-help"),   icon: Icon::Support},
    ];
    view! {
        <footer class="bg-main-background w-full h-12 flex flex-wrap justify-end md:grid md:grid-cols-[10%_80%_10%]">
            <div class="md:col-start-2 md:col-end-3 w-full flex justify-between sm:justify-end p-4">    
                {links.into_iter()
                    .map(|link| view! {
                        <a class="ml-4 flex items-center text-white text-xs uppercase hover:text-granola-orange hover:underline" href=link.href>
                            {match link.icon {
                                Icon::Docs => view! { <DocsIcon width=12/> },
                                Icon::Feedback => view! { <FeedbackIcon width=12 /> },
                                Icon::Terms => view! { <TermsIcon width=12 /> },
                                Icon::Support => view! { <SupportIcon width=12 /> },
                            }}
                            <div class="ml-1">{link.label}</div>
                        </a>
                    }).collect::<Vec<_>>()}
            </div>
        </footer>
    }
}