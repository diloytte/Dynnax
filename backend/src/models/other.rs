use core::fmt;
use std::{io, path::Display, sync::Arc};

use axum::Extension;
use serde::Serialize;

use crate::state::AppState;


#[derive(Debug,Serialize)]
pub enum Amount {
    Float(f32),
    String(String),
}

#[derive(Debug, Serialize)]
pub struct TradeRequestBuy {
    pub action: String,
    pub mint: String,
    pub amount: Amount,
    #[serde(rename = "denominatedInSol")]
    pub denominated_in_sol: String,
    pub slippage: i32,
    #[serde(rename = "priorityFee")]
    pub priority_fee: f32,
    pub pool: String,
}

pub type AppStateExtension = Extension<Arc<AppState>>;

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