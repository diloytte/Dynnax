use crate::models::{
    other::{MyRecvBody, TradeRequest},
    service::snipe_target::SnipeTarget,
};

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
        denominated_in_sol: true,
        slippage: snipe_target.snipe_config.slippage,
        priority_fee: snipe_target.snipe_config.priority_fee,
        pool: "auto".to_string(),
    };

    let url = format!("{}{}", PUMP_PORTAL_URL, api_key);

    let recv_body = ureq::post(url)
        .send_json(&body)?
        .body_mut()
        .read_json::<MyRecvBody>()?;

    dbg!(recv_body);

    Ok(())
}
