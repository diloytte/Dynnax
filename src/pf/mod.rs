use serde_json::Value;

use crate::{models::{other::{Browser, TradeRequest}, pf::PfResponse, service::snipe_target::SnipeTarget}, utils::open_browser};

static PUMP_PORTAL_URL: &str = "https://pumpportal.fun/api/trade?api-key=";

pub async fn buy_ca(
    api_key: &str,
    snipe_target: &SnipeTarget,
    ca: String,
) -> Result<(), ureq::Error> {
    let body = TradeRequest {
        action: "buy".to_string(),
        mint: ca.to_string(),
        amount: snipe_target.snipe_config.sol_amount,
        denominated_in_sol: "true".to_string(),
        slippage: snipe_target.snipe_config.slippage,
        priority_fee: snipe_target.snipe_config.priority_fee,
        pool: "auto".to_string(),
    };

    let url = format!("{}{}", PUMP_PORTAL_URL, api_key);

    let pf_response: PfResponse = ureq::post(url).send_json(&body)?.body_mut().read_json()?;

    match pf_response.signature {
        Some(sig) => {
            println!("Transaction sent. Signature: {}", sig);
            open_browser(Browser::Brave, format!("https://neo.bullx.io/terminal?chainId=1399811149&address={}",ca));
        },
        None => {
            for error in &pf_response.errors {
                println!("PumpFun error: {}", error);
            }
        }
    }

    Ok(())
}
