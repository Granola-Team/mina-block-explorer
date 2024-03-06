use leptos::{html::Textarea, web_sys::SubmitEvent, *};

#[derive(Clone)]
struct ActionInputs {
    value: String,
    endpoint: String,
}

#[component]
pub fn BroadcastForm(endpoint: String) -> impl IntoView {
    let textarea_element: NodeRef<Textarea> = create_node_ref();

    let submit_action = create_action(|input: &ActionInputs| {
        let input = input.clone();
        async move {
            reqwest::Client::new()
                .post(&input.endpoint)
                .body(input.value)
                .send()
                .await
        }
    });

    view! {
        <form
            class="p-8"
            on:submit=move |ev: SubmitEvent| {
                ev.prevent_default();
                let value = textarea_element.get().expect("<textarea> to exist").value();
                submit_action
                    .dispatch(ActionInputs {
                        value,
                        endpoint: endpoint.clone(),
                    })
            }
        >

            <pre>
                <textarea
                    class="p-4 border-box w-full border border-[#DADCE0] rounded-md"
                    node_ref=textarea_element
                    rows="10"
                ></textarea>
            </pre>
            <input
                disabled=move || submit_action.pending().get()
                type="submit"
                class="disabled:bg-slate-400 disabled:text-slate-200 disabled:cursor-not-allowed bg-granola-orange text-white uppercase h-11 rounded-lg px-6 cursor-pointer disabled:cursor-not-allowed"
                value="Send"
            />
        </form>
    }
}

#[component]
pub fn Sample(children: Children) -> impl IntoView {
    view! {
        <div class="w-full overflow-x-auto">
            <pre class="m-8 p-4 border-box stretch w-fit border border-[#DADCE0] rounded-md">
                {children()}
            </pre>
        </div>
    }
}
