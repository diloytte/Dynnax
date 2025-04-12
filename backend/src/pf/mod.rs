use crate::{
    models::{
        other::{Amount, Browser, TradeRequestBuy},
        pf::PfResponse,
        service::snipe_target::SnipeTarget,
    },
    utils::{open_browser, play_buy_notif},
};

static PUMP_PORTAL_URL: &str = "https://pumpportal.fun/api/trade?api-key=";

#[derive(Debug)]
pub enum TradeError {
    UreqError(ureq::Error),
    CustomError(String),
}

impl From<ureq::Error> for TradeError {
    fn from(err: ureq::Error) -> Self {
        TradeError::UreqError(err)
    }
}

pub async fn buy_ca(api_key: &str, snipe_target: &SnipeTarget, ca: String) -> Result<(), TradeError> {
    let body = TradeRequestBuy {
        action: "buy".to_string(),
        mint: ca.to_string(),
        amount: Amount::Float(snipe_target.snipe_config.sol_amount),
        denominated_in_sol: "true".to_string(),
        slippage: snipe_target.snipe_config.slippage,
        priority_fee: snipe_target.snipe_config.priority_fee,
        pool: "auto".to_string(),
    };

    let url = format!("{}{}", PUMP_PORTAL_URL, api_key);

    let pf_response: PfResponse = ureq::post(url).send_json(&body)?.body_mut().read_json()?;

    match pf_response.signature {
        Some(sig) => {
            println!("BUY Transaction sent. Signature: {}", sig);
            play_buy_notif();
            let _ = open_browser(
                Browser::Brave,
                format!(
                    "https://neo.bullx.io/terminal?chainId=1399811149&address={}",
                    ca
                ),
            );
        }
        None => {
            return Err(TradeError::CustomError(
                format!("{} \n CA: {}",pf_response.errors.first().unwrap().to_string(),ca)
            )); //Return just the first error, just for the sake of returning it...
        }
    }

    Ok(())
}

pub async fn sell_ca(api_key:&str,snipe_target:&SnipeTarget,ca:String,sell_precent:f32)->Result<(),TradeError>{
    let body = TradeRequestBuy {
        action: "sell".to_string(),
        mint: ca.to_string(),
        amount: Amount::String(format!("{}%",sell_precent)),
        denominated_in_sol: "false".to_string(),
        slippage: snipe_target.snipe_config.slippage,
        priority_fee: snipe_target.snipe_config.priority_fee,
        pool: "auto".to_string(),
    };

    let url = format!("{}{}", PUMP_PORTAL_URL, api_key);
    let pf_response: PfResponse = ureq::post(url).send_json(&body)?.body_mut().read_json()?;

    match pf_response.signature {
        Some(sig) => {
            println!("SELL Transaction sent. Signature: {}", sig);
        }
        None => {
            return Err(TradeError::CustomError(
                format!("{} \n CA: {}",pf_response.errors.first().unwrap().to_string(),ca)
            )); //Return just the first error, just for the sake of returning it...
        }
    }

    Ok(())
}

