use crate::state::AppState;
use grammers_client::types::Message;
use grammers_client::{Client, InvocationError};
use std::sync::Arc;

pub async fn snipe_x(
    message: &Message,
    client: &Client,
    shared_state: &Arc<AppState>,
    ca: &String,
) -> Result<(), InvocationError> {
    Ok(())
}
