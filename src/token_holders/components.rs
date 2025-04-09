use crate::{
    common::{components::*, functions::*, models::*, spotlight::*, table::*},
    token_holders::graphql::token_holders_query::*,
};
use leptos::*;

#[component]
pub fn TokenHoldersMoreDetails(zkapp: TokenHoldersQueryTokenHoldersAccountZkapp) -> impl IntoView {
    let (metadata, _) = create_signal::<Option<TableMetadata>>(None);
    let zkapp_clone = zkapp.clone();
    view! {
        <TableSection metadata=metadata.into() section_heading="More Details">
            <SpotlightTable>
                <ZkAppDetailTr>
                    <ZkAppDetailTh>"Ver. Key Hash:"</ZkAppDetailTh>
                    <ZkAppDetailTd>
                        {zkapp.verification_key.map(|vk| vk.hash.unwrap_or("None".to_string()))}
                    </ZkAppDetailTd>
                </ZkAppDetailTr>
                <ZkAppDetailTr>
                    <ZkAppDetailTh>"Zkapp URI:"</ZkAppDetailTh>
                    <ZkAppDetailTd>
                        {zkapp.zkapp_uri.map(|uri| uri.to_string()).unwrap_or("None".to_string())}
                    </ZkAppDetailTd>
                </ZkAppDetailTr>
                <ZkAppDetailTr>
                    <ZkAppDetailTh>"App State:"</ZkAppDetailTh>
                    <ZkAppDetailTd>
                        <CopyToClipboard>
                            <CodeBlock>
                                {format_json_array_pretty(zkapp.app_state.unwrap_or_default())}
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
                                    zkapp_clone.action_state.unwrap_or_default(),
                                )}
                            </CodeBlock>
                        </CopyToClipboard>
                    </ZkAppDetailTd>
                </ZkAppDetailTr>
            </SpotlightTable>
        </TableSection>
    }
}
