use crate::token_holders::graphql::token_holders_query::TokenHoldersQueryTokenHoldersAccountPermissions;

impl TokenHoldersQueryTokenHoldersAccountPermissions {
    pub fn to_key_value_pairs(&self) -> Vec<Option<String>> {
        vec![
            Some(format!("edit_state:{}", self.edit_state)),
            Some(format!("access:{}", self.access)),
            Some(format!("send:{}", self.send)),
            Some(format!("receive:{}", self.receive)),
            Some(format!("set_delegate:{}", self.set_delegate)),
            Some(format!("set_permissions:{}", self.set_permissions)),
            // Some(format!("set_verification_key:{}", self.set_verification_key.to_string())),
            Some(format!("set_zkapp_uri:{}", self.set_zkapp_uri)),
            Some(format!("edit_action_state:{}", self.edit_action_state)),
            Some(format!("set_token_symbol:{}", self.set_token_symbol)),
            Some(format!("increment_nonce:{}", self.increment_nonce)),
            Some(format!("set_voting_for:{}", self.set_voting_for)),
            Some(format!("set_timing:{}", self.set_timing)),
        ]
    }
}
