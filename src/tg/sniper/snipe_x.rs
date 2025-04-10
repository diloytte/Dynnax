use crate::state::AppState;
use grammers_client::types::Message;
use grammers_client::{Client, InvocationError, Update};
use std::sync::Arc;
use token_address_extractor::extract_solana_address;

pub async fn snipe_x(
    message: &Message,
    client: &Client,
    shared_state: &Arc<AppState>,
    pf_api_key: &String,
    ca:&String
) -> Result<(), InvocationError> {
    Ok(())
}