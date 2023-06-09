use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::validators::is_lowercase_alphanumeric;

#[derive(Debug, Deserialize, Validate)]
pub struct SignupForm {
    #[validate(length(min = 4, max = 32), custom = "is_lowercase_alphanumeric")]
    pub username: String,
    #[validate(email)]
    pub email: String,
    #[validate(length(min = 8), must_match(other = "repeat_password"))]
    pub password: String,
    repeat_password: String,
}

#[derive(Debug, Deserialize, Validate)]
pub struct LoginForm {
    #[validate(length(min = 4, max = 32), custom = "is_lowercase_alphanumeric")]
    pub username: String,
    #[validate(length(min = 8))]
    pub password: String,
}

#[derive(Debug, Serialize)]
pub struct AuthBody {
    access_token: String,
    token_type: String,
}

impl AuthBody {
    pub fn new(access_token: String) -> Self {
        Self {
            access_token,
            token_type: "Bearer".to_string(),
        }
    }
}
