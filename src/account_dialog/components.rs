use leptos::*;

use crate::common::models::*;

#[component]
pub fn AccountDialogSectionContainer(
    title: String,
    showing_message: String,
    children: Children,
) -> impl IntoView {
    view! {
        <section class="flex flex-col bg-white rounded-xl flex flex-col items-stretch mt-8 p-4 h-fit">
            <div class="flex justify-between w-full mb-4">
                <h2 class="text-xl">{title}</h2>
                <span class="text-table-row-text-color text-xs">{showing_message}</span>
            </div>
            {children()}
        </section>
    }.into_view()
}

#[component]
pub fn AccountDialogSectionSubEntry(label: String, value: String) -> impl IntoView {
    match label.len() {
        0 => view! { <div></div> },
        _ => view! {
            <div class="w-1/2 flex my-1">
                <span class="text-xs text-slate-400 w-1/4">{label} :</span>
                <span class="text-xs overflow-hidden text-ellipsis w-3/4">{value}</span>
            </div>
        },
    }
}

pub struct StatusImg<'a> {
    src: &'a str,
    alt: &'a str,
}

#[component]
pub fn AccountDialogSectionEntryHeader(
    date: String,
    status: Status,
    moments_ago: String,
) -> impl IntoView {
    let img_attr = match status {
        Status::Pending => StatusImg {
            src: "/img/timelapse.svg",
            alt: "Pending",
        },
        Status::Complete => StatusImg {
            src: "/img/success.svg",
            alt: "Complete",
        },
        Status::Unknown => StatusImg {
            src: "",
            alt: "Unknown",
        },
    };
    view! {
        <div class="flex justify-between w-full">
            <div class="flex items-center">
                <img src=img_attr.src alt=img_attr.alt class="mr-2"/>
                {move || match status {
                    Status::Complete => {
                        view! { <span class="text-sm">{date.clone()}</span> }.into_view()
                    }
                    Status::Pending => view! { <span class="text-sm">"Pending"</span> }.into_view(),
                    Status::Unknown => view! { <span class="text-sm">"Unknown"</span> }.into_view(),
                }}

            </div>
            <div class="text-xs text-slate-400">{moments_ago}</div>
        </div>
    }
}

#[component]
pub fn AccountDialogEntryDivider() -> impl IntoView {
    view! { <div class="border-b border-slate-100 my-2 h-1 w-full"></div> }
}
