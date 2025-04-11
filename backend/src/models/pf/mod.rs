use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct ErrorResponse {
    pub errors: Vec<String>,
}

#[derive(Debug, Deserialize)]
pub struct PfResponse {
    pub errors: Vec<String>,
    pub signature: Option<String>,
}
