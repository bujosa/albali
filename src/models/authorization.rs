use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct AuthorizationResponse {
    access_token: String,
    token_type: String,
    expires_in: u32,
}

impl AuthorizationResponse {
    pub fn new(access_token: String, token_type: String, expires_in: u32) -> Self {
        Self {
            access_token,
            token_type,
            expires_in,
        }
    }

    pub fn get_access_token(&self) -> &String {
        &self.access_token
    }
}

impl Default for AuthorizationResponse {
    fn default() -> Self {
        Self {
            access_token: String::new(),
            token_type: String::new(),
            expires_in: 0,
        }
    }
}
