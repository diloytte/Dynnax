use crate::{pf::buy_ca, state::AppState};
use grammers_client::{Client, InvocationError, Update};
use std::sync::Arc;
use token_address_extractor::{extract_solana_address, extract_token_address_from_message_text};
use tokio::sync::RwLock;

pub async fn snipe_dialogs(
    client: Client,
    shared_state: Arc<RwLock<AppState>>,
    pf_api_key: String,
) -> Result<(), InvocationError> {
    todo!()
}