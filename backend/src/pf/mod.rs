use serde::Serialize;

use crate::{
    models::{
        other::{Browser, TradeRequestBuy, TradeRequestSell},
        pf::PfResponse,
        service::snipe_target::SnipeTarget,
    },
    utils::{open_browser, play_buy_notif},
};

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

pub async fn manual_buy_token(url:&str,sol_amount:f32,ca:String)->Result<(),TradeError>{
    let body = TradeRequestBuy {
        action: "buy".to_string(),
        mint: ca.to_string(),
        amount: sol_amount,
        denominated_in_sol: "true".to_string(),
        slippage: 15,
        priority_fee: 0.0005,
        pool: "auto".to_string(),
    };

    let pf_response: PfResponse = ureq::post(url).send_json(&body)?.body_mut().read_json()?;

    match pf_response.signature {
        Some(sig) => {
            println!("BUY Transaction sent. Signature: {}", sig);
            play_buy_notif();
        }
        None => {
            return Err(TradeError::CustomError(
                format!("{} \n CA: {}",pf_response.errors.first().unwrap().to_string(),ca)
            )); //Return just the first error, just for the sake of returning it...
        }
    }

    Ok(())
}

pub async fn buy_ca(url: &str, snipe_target: &SnipeTarget, ca: String) -> Result<(), TradeError> {
    let body = TradeRequestBuy {
        action: "buy".to_string(),
        mint: ca.to_string(),
        amount: snipe_target.snipe_config.sol_amount,
        denominated_in_sol: "true".to_string(),
        slippage: snipe_target.snipe_config.slippage,
        priority_fee: snipe_target.snipe_config.priority_fee,
        pool: "auto".to_string(),
    };

    let pf_response: PfResponse = ureq::post(url).send_json(&body)?.body_mut().read_json()?;

    match pf_response.signature {
        Some(sig) => {
            println!("BUY Transaction sent. Signature: {}", sig);
            play_buy_notif();
        }
        None => {
            return Err(TradeError::CustomError(
                format!("{} \n CA: {}",pf_response.errors.first().unwrap().to_string(),ca)
            )); //Return just the first error, just for the sake of returning it...
        }
    }

    Ok(())
}

pub async fn sell_ca(url:&str,snipe_target:&SnipeTarget,ca:String,sell_precent:f32)->Result<(),TradeError>{
    let body = TradeRequestSell {
        action: "sell".to_string(),
        mint: ca.to_string(),
        amount: format!("{}%",sell_precent.to_string()),
        denominated_in_sol: "false".to_string(),
        slippage: snipe_target.snipe_config.slippage,
        priority_fee: snipe_target.snipe_config.priority_fee,
        pool: "auto".to_string(),
    };

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

