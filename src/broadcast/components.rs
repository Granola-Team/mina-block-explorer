use leptos::*;
use leptos::{html::Textarea, web_sys::{SubmitEvent}};

#[component]
pub fn BroadcastForm(endpoint: String) -> impl IntoView {

    let (text, _set_text) = create_signal("".to_string());
    let textarea_element: NodeRef<Textarea> = create_node_ref();

    let on_submit = move |ev: SubmitEvent| {
        // stop the page from reloading!
        ev.prevent_default();
    
        let value = textarea_element.get()
            .expect("<textarea> to exist")
            .value();
        logging::log!("Broadcasting {} to {}",value.clone(), endpoint);
    };

    view! {
        <form on:submit=on_submit class="p-8">
            <code>
                <textarea prop:value=move || text.get()
                    class="p-2 border-box w-full border border-[#DADCE0] rounded-md"
                    node_ref=textarea_element
                    rows="10">
                    /* untracked, plain-text initial value */
                    {untrack(move || text.get())}
                </textarea>
            </code>
            <input type="submit" class="disabled:bg-slate-400 disabled:text-slate-200 disabled:cursor-not-allowed bg-granola-orange text-white uppercase h-11 rounded-lg px-6" value="Send"/>
        </form>
    }
}