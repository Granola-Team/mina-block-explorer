use super::functions::load_data;
use crate::{
    common::{
        components::{CodeBlock, CopyToClipboard, ZkAppDetailTd, ZkAppDetailTh, ZkAppDetailTr},
        functions::*,
        models::{ColorVariant, TableMetadata},
        spotlight::{SpotlightEntry, SpotlightSection, SpotlightTable},
        table::TableSection,
    },
    icons::TokenSymbol,
};
use leptos::*;
use leptos_meta::Title;
use leptos_router::use_params_map;

#[component]
pub fn TokenHolderPage() -> impl IntoView {
    let (metadata, _) = create_signal::<Option<TableMetadata>>(None);
    let memo_params_map = use_params_map();
    let (token_symbol_sig, set_token) = create_signal::<String>("".to_string());

    let resource = create_resource(
        move || {
            (
                memo_params_map
                    .get()
                    .get("id")
                    .cloned()
                    .expect("Account is required"),
                memo_params_map
                    .get()
                    .get("token_id")
                    .cloned()
                    .expect("Token ID is required"),
            )
        },
        move |(account, token_id)| async move {
            load_data(account.to_string(), token_id.to_string()).await
        },
    );

    let get_token = move || {
        resource
            .get()
            .and_then(|res| res.ok())
            .and_then(|rd| rd.token_holders.first().cloned().flatten())
    };

    create_effect(move |_| {
        if let Some(token) = get_token() {
            set_token.set(token.symbol.to_string());
        }
    });

    view! {
        <Title
            formatter=move |text| format!("Token Holding | {text}")
            text=move || {
                format!(
                    "{} | {}",
                    memo_params_map.get().get("id").cloned().expect("Account is required"),
                    token_symbol_sig.get(),
                )
            }
        />
        {move || match get_token() {
            Some(token) => {
                let zk_app_uri = token
                    .account
                    .zkapp
                    .as_ref()
                    .and_then(|zkapp| zkapp.zkapp_uri.clone())
                    .unwrap_or("None".to_string());
                let verification_key_hash = token
                    .account
                    .zkapp
                    .as_ref()
                    .and_then(|zkapp| zkapp.verification_key.as_ref())
                    .and_then(|key| key.hash.clone())
                    .unwrap_or("None".to_string());
                let zkapp_clone = token.account.zkapp.clone();
                let zkapp = token.account.zkapp.clone();
                // C
                // C
                view! {
                    <SpotlightSection
                        header="Token Holding"
                        spotlight_items=vec![
                            SpotlightEntry {
                                label: "Public Key".to_string(),
                                any_el: Some(
                                    convert_to_copy_link(
                                        token.account.public_key.to_string(),
                                        format!("/addresses/accounts/{}", token.account.public_key),
                                    ),
                                ),
                                ..Default::default()
                            },
                            SpotlightEntry {
                                label: "Balance".to_string(),
                                any_el: Some(
                                    convert_to_span(
                                        format_number(token.account.balance.to_string()),
                                    ),
                                ),
                                ..Default::default()
                            },
                            SpotlightEntry {
                                label: "Nonce".to_string(),
                                any_el: Some(
                                    convert_to_pill(
                                        token.account.nonce.to_string(),
                                        ColorVariant::Grey,
                                    ),
                                ),
                                ..Default::default()
                            },
                            SpotlightEntry {
                                label: "Delegate".to_string(),
                                any_el: Some(
                                    convert_to_copy_link(
                                        token.account.delegate.to_string(),
                                        format!("/addresses/accounts/{}", token.account.delegate),
                                    ),
                                ),
                                ..Default::default()
                            },
                            SpotlightEntry {
                                label: "Zkapp URI".to_string(),
                                any_el: Some(convert_to_link(zk_app_uri.to_string(), zk_app_uri)),
                                ..Default::default()
                            },
                            SpotlightEntry {
                                label: "Ver. Hash Key".to_string(),
                                any_el: Some(convert_to_span(verification_key_hash)),
                                ..Default::default()
                            },
                        ]
                        meta=Some(format!("Symbol: {}", token.symbol))
                        id=memo_params_map.get().get("token_id").cloned()
                    >
                        <TokenSymbol width=40 />
                    </SpotlightSection>
                    <TableSection metadata=metadata.into() section_heading="Other Zkapp Details">
                        <SpotlightTable>
                            <ZkAppDetailTr>
                                <ZkAppDetailTh>"App State:"</ZkAppDetailTh>
                                <ZkAppDetailTd>
                                    <CopyToClipboard>
                                        <CodeBlock>
                                            {format_json_array_pretty(
                                                zkapp
                                                    .as_ref()
                                                    .and_then(|zkapp| zkapp.app_state.clone())
                                                    .unwrap_or(vec![]),
                                            )}
                                        </CodeBlock>
                                    </CopyToClipboard>
                                </ZkAppDetailTd>
                            </ZkAppDetailTr>
                            <ZkAppDetailTr>
                                <ZkAppDetailTh>"Action State:"</ZkAppDetailTh>
                                <ZkAppDetailTd>
                                    <CopyToClipboard>
                                        <CodeBlock>
                                            {format_json_array_pretty(
                                                zkapp_clone
                                                    .as_ref()
                                                    .and_then(|zkapp| zkapp.action_state.clone())
                                                    .unwrap_or(vec![]),
                                            )}
                                        </CodeBlock>
                                    </CopyToClipboard>
                                </ZkAppDetailTd>
                            </ZkAppDetailTr>
                        </SpotlightTable>
                    </TableSection>
                }
                    .into_view()
            }
            None => {
                view! {
                    <SpotlightSection
                        header="Token Overview"
                        spotlight_items=vec![]
                        meta=None
                        id=None
                    >
                        <TokenSymbol width=40 />
                    </SpotlightSection>
                }
            }
        }}
    }
}
