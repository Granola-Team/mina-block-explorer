use crate::{
    common::{components::*, functions::*, models::*, spotlight::*, table::*},
    token_holders::graphql::token_holders_query::*,
};
use leptos::*;

#[component]
pub fn TokenHoldersMoreDetails(
    zkapp: Option<TokenHoldersQueryTokenHoldersAccountZkapp>,
    permissions: Option<TokenHoldersQueryTokenHoldersAccountPermissions>,
) -> impl IntoView {
    let (metadata, _) = create_signal::<Option<TableMetadata>>(None);
    if zkapp.is_none() && permissions.is_none() {
        ().into_view()
    } else {
        view! {
            <TableSection metadata=metadata.into() section_heading="More Details">
                <SpotlightTable id=MaybeSignal::derive(|| {
                    "More Details".to_string()
                })>
                    {zkapp
                        .map(|zkapp| {
                            view! {
                                <ZkAppDetailTr>
                                    <ZkAppDetailTh>"App State:"</ZkAppDetailTh>
                                    <ZkAppDetailTd>
                                        <CopyToClipboard>
                                            <CodeBlock>
                                                {format_json_array_pretty(
                                                    zkapp.app_state.unwrap_or_default(),
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
                                                    zkapp.action_state.unwrap_or_default(),
                                                )}
                                            </CodeBlock>
                                        </CopyToClipboard>
                                    </ZkAppDetailTd>
                                </ZkAppDetailTr>
                            }
                        })}
                    {permissions
                        .map(|permissions| {
                            view! {
                                <ZkAppDetailTr>
                                    <ZkAppDetailTh>"Permissions:"</ZkAppDetailTh>
                                    <ZkAppDetailTd>
                                        <CopyToClipboard>
                                            <CodeBlock>
                                                {format_json_array_pretty(permissions.to_key_value_pairs())}
                                            </CodeBlock>
                                        </CopyToClipboard>
                                    </ZkAppDetailTd>
                                </ZkAppDetailTr>
                            }
                        })}
                </SpotlightTable>
            </TableSection>
        }
    }
}
