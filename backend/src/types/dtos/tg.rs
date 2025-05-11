use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct IgnoreUserDTO {
    pub ignore_user: Option<bool>,
}

#[derive(Debug, Deserialize)]
pub struct ShillDTO{
    pub shill_message: String,
    pub is_test: bool
}