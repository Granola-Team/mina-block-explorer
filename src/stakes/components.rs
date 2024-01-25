use super::models::*;
use leptos::*;
use leptos_router::*;

#[component]
pub fn StakesNavButton(href: String, text: String) -> impl IntoView {
    view! {
        <a href=href class="cursor-pointer text-sm rounded-md p-2 h-9 font-semibold mx-2 flex justify-center items-center border border-granola-orange border-[1px] text-white bg-granola-orange">
            {text}
        </a>
    }
}

#[component]
pub fn EpochButton(
    text: String,
    epoch_target: i64,
    #[prop(default = false)] disabled: bool,
    style_variant: EpochStyleVariant,
) -> impl IntoView {
    let button_base_styles = "text-sm rounded-md p-2 h-9 font-semibold mx-2 flex justify-center items-center border border-granola-orange border-[1px]";
    let mut button_variant_styles = match style_variant {
        EpochStyleVariant::Primary => {
            format!("{} {}", button_base_styles, "text-white bg-granola-orange")
        }
        EpochStyleVariant::Secondary => {
            format!("{} {}", button_base_styles, "text-granola-orange bg-white")
        }
    };
    button_variant_styles = match disabled {
        true => format!(
            "{} {}",
            button_variant_styles, "bg-slate-400 border-slate-400 hover:cursor-not-allowed"
        ),
        false => button_variant_styles,
    };

    let query_params_map = use_query_map();
    let navigate = use_navigate();
    let location = use_location();

    let handle_click = move |_| {
        let pathname = location.pathname.get();
        let mut pm = query_params_map.get();
        pm.insert("epoch".to_string(), epoch_target.to_string());

        logging::log!("{}", pm.to_query_string());
        logging::log!("{}", pathname);

        navigate(
            &format!("{}{}", pathname, pm.to_query_string()),
            NavigateOptions {
                resolve: true,
                replace: false,
                scroll: false,
                state: State(None),
            },
        );
    };

    view! {
        <button on:click=handle_click class=button_variant_styles>
            {text}
        </button>
    }
}
