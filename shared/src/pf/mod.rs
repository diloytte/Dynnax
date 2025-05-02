use grammers_client::types::Chat;
use grammers_client::{Client, InvocationError};
use reqwest::Client as ReqwestClient;

use crate::types::{DexScreenerResponse, SnipeConfig, TradeRequest};
use crate::types::{PfResponse, SnipeTarget, TradeError, TradeRequestBuy, TradeRequestSell};

#[cfg(not(feature = "remote"))]
use crate::utils::play_buy_notif;

#[cfg(feature = "performance_log")]
use std::time::Instant;

pub async fn manual_buy_token(
    url: &str,
    sol_amount: f32,
    ca: String,
    http_client: &ReqwestClient,
) -> Result<(), TradeError> {
    let body = TradeRequestBuy {
        action: "buy".to_string(),
        mint: ca.to_string(),
        amount: sol_amount,
        denominated_in_sol: "true".to_string(),
        slippage: 15,
        priority_fee: 0.0005,
        pool: "auto".to_string(),
    };

    let response = http_client.post(url).json(&body).send().await?;

    let pf_response: PfResponse = response.json().await?;

    match pf_response.signature {
        Some(sig) => {
            println!("BUY Transaction sent. Signature: {}", sig);
            #[cfg(not(feature = "remote"))]
            play_buy_notif();
        }
        None => {
            return Err(TradeError::CustomError(format!(
                "{} \n CA: {}",
                pf_response.errors.first().unwrap(),
                ca
            ))); //Return just the first error, just for the sake of returning it...
        }
    }

    Ok(())
}

pub async fn buy_ca_tg(client: &Client, message: &str, chat: &Chat) -> Result<(), InvocationError> {
    client.send_message(chat, message).await?;
    Ok(())
}

pub async fn buy_ca(
    url: &str,
    snipe_config: &SnipeConfig,
    ca: &str,
    fee_multiplier: u8,
    http_client: &ReqwestClient,
) -> Result<(), TradeError> {
    #[cfg(feature = "performance_log")]
    let start = Instant::now();

    let body = TradeRequestBuy {
        action: "buy".to_string(),
        mint: ca.to_string(),
        amount: snipe_config.sol_amount,
        denominated_in_sol: "true".to_string(),
        slippage: snipe_config.slippage,
        priority_fee: snipe_config.priority_fee * fee_multiplier as f32,
        pool: "auto".to_string(),
    };

    match send_trade(&TradeRequest::Buy(&body), url, http_client).await {
        Ok(_) => {
            #[cfg(feature = "performance_log")]
            {
                let duration = start.elapsed();
                println!("Buy delay: {} ms", duration.as_millis());
            }
        }
        Err(error) => {
            println!("Error: {:?}, CA: {}", error, ca);
            let dex_address = fetch_base_token_address_from_dex(ca, http_client).await;
            if dex_address.is_err() {
                println!("Error from DEX. {:?}", error);
                return Err(TradeError::CustomError(dex_address.err().unwrap()));
            }
            send_dex_ca_trade(ca, url, &body, http_client).await?;
        }
    }

    Ok(())
}

pub async fn sell_ca(
    url: &str,
    snipe_target: &SnipeTarget,
    ca: String,
    sell_precent: f32,
    http_client: &ReqwestClient,
) -> Result<(), TradeError> {
    let body = TradeRequestSell {
        action: "sell".to_string(),
        mint: ca.to_string(),
        amount: format!("{}%", sell_precent),
        denominated_in_sol: "false".to_string(),
        slippage: snipe_target.snipe_config.slippage,
        priority_fee: snipe_target.snipe_config.priority_fee,
        pool: "auto".to_string(),
    };
    let response = http_client.post(url).json(&body).send().await?;

    let pf_response: PfResponse = response.json().await?;
    match pf_response.signature {
        Some(sig) => {
            println!("SELL Transaction sent. Signature: {}", sig);
        }
        None => {
            return Err(TradeError::CustomError(format!(
                "{} \n CA: {}",
                pf_response.errors.first().unwrap(),
                ca
            ))); //Return just the first error, just for the sake of returning it...
        }
    }

    Ok(())
}

pub async fn send_trade(
    body: &TradeRequest<'_>,
    url: &str,
    http_client: &ReqwestClient,
) -> Result<(), TradeError> {
    let response = http_client
        .post(url)
        .json(&body.to_payload())
        .send()
        .await?;

    let pf_response: PfResponse = response.json().await?;
    match pf_response.signature {
        Some(sig) => {
            println!("Transaction sent. Signature: {}", sig);
        }
        None => {
            return Err(TradeError::CustomError(format!(
                "{} \n",
                pf_response.errors.first().unwrap()
            ))); //Return just the first error, just for the sake of returning it...
        }
    }
    Ok(())
}

pub async fn fetch_base_token_address_from_dex(
    ca: &str,
    http_client: &ReqwestClient,
) -> Result<String, String> {
    let url = format!("https://api.dexscreener.com/latest/dex/pairs/solana/{}", ca);

    let response = match http_client.get(&url).send().await {
        Ok(resp) => resp,
        Err(e) => {
            println!("Error sending request to dexscreener: {:?}", e);
            return Err("Error in response from dexscreener.".to_string());
        }
    };

    let response_body_data: DexScreenerResponse = match response.json().await {
        Ok(parsed) => parsed,
        Err(e) => {
            println!("Error parsing response body: {:?}", e);
            return Err("Error parsing response body from dexscreener.".to_string());
        }
    };

    if let Some(pair) = response_body_data.pairs.first() {
        Ok(pair.base_token.address.clone())
    } else {
        Err(format!("No pair found for ca: {}", ca))
    }
}

async fn send_dex_ca_trade(
    ca: &str,
    url: &str,
    body: &TradeRequestBuy,
    http_client: &ReqwestClient,
) -> Result<(), TradeError> {
    let dex_address = fetch_base_token_address_from_dex(ca, http_client).await;
    if let Err(err) = dex_address {
        println!("Error from DEX. {:?}", err);
        return Err(TradeError::CustomError(err));
    }

    let dex_mint = dex_address.unwrap();
    let dex_body = TradeRequestBuy {
        action: body.action.clone(),
        mint: dex_mint,
        amount: body.amount,
        denominated_in_sol: body.denominated_in_sol.clone(),
        slippage: body.slippage,
        priority_fee: body.priority_fee,
        pool: body.pool.clone(),
    };

    match send_trade(&TradeRequest::Buy(&dex_body), url, http_client).await {
        Ok(_) => Ok(()),
        Err(error) => Err(TradeError::CustomError(format!(
            "Error: {:?}, CA: {}",
            error, ca
        ))),
    }
}
