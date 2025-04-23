use std::io;
use core::fmt;

use serde::{Deserialize, Serialize};

pub enum Browser {
    Brave,
}

#[derive(Debug)]
pub enum SoundError {
    IoError(io::Error),
    StreamError(rodio::StreamError),
    DecoderError(rodio::decoder::DecoderError)
}

impl From<io::Error> for SoundError {
    fn from(err: io::Error) -> Self {
        SoundError::IoError(err)
    }
}

impl From<rodio::StreamError> for SoundError{
    fn from(err: rodio::StreamError) -> Self {
        SoundError::StreamError(err)
    }
}

impl From<rodio::decoder::DecoderError> for SoundError{
    fn from(err: rodio::decoder::DecoderError) -> Self {
        SoundError::DecoderError(err)
    }
}

impl fmt::Display for SoundError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            SoundError::IoError(err) => write!(f, "I/O error: {}", err),
            SoundError::StreamError(err) => write!(f, "Stream error: {}", err),
            SoundError::DecoderError(err) => write!(f, "Decoder error. {}",err),
        }
    }
}

















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

#[derive(Debug, Serialize)]
pub struct TradeRequestBuy {
    pub action: String,
    pub mint: String,
    pub amount: f32,
    #[serde(rename = "denominatedInSol")]
    pub denominated_in_sol: String,
    pub slippage: i32,
    #[serde(rename = "priorityFee")]
    pub priority_fee: f32,
    pub pool: String,
}

#[derive(Debug, Serialize)]
pub struct TradeRequestSell {
    pub action: String,
    pub mint: String,
    pub amount: String,
    #[serde(rename = "denominatedInSol")]
    pub denominated_in_sol: String,
    pub slippage: i32,
    #[serde(rename = "priorityFee")]
    pub priority_fee: f32,
    pub pool: String,
}


#[derive(Debug, Deserialize)]
pub struct ErrorResponse {
    pub errors: Vec<String>,
}

#[derive(Debug, Deserialize)]
pub struct PfResponse {
    pub errors: Vec<String>,
    pub signature: Option<String>,
}










































#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct SnipeConfig {
    pub sol_amount: f32,
    pub slippage: i32,
    pub priority_fee: f32,
}

impl Default for SnipeConfig {
    fn default() -> Self {
        SnipeConfig {
            sol_amount: 1.0,
            slippage: 15,
            priority_fee: 0.005,
        }
    }
}

#[derive(Debug, Serialize, Clone)]
pub struct SnipeTarget {
    pub target_name: String,
    pub snipe_config: SnipeConfig,
    pub is_active: bool,
    pub deactivate_on_snipe: bool,
    pub past_shills: Vec<String>,
}

impl Default for SnipeTarget {
    fn default() -> Self {
        SnipeTarget {
            target_name: String::from("None"),
            snipe_config: SnipeConfig::default(),
            is_active: true,
            deactivate_on_snipe: true,
            past_shills: Vec::new(),
        }
    }
}

#[derive(Debug, Serialize)]
pub struct TwitterTarget {
    pub snipe_config: SnipeConfig,
    pub is_active: bool,
    pub deactivate_on_snipe: bool,
}

impl Default for TwitterTarget {
    fn default() -> Self {
        Self {
            snipe_config: Default::default(),
            is_active: true,
            deactivate_on_snipe: true,
        }
    }
}
