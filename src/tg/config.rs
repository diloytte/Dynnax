// src/config.rs
use std::env;
use tokio::fs as tokfs;
use crate::Session;
use std::error::Error;

#[derive(Debug)]
pub struct ConfigData {
    pub api_id: i32,
    pub api_hash: String,
    pub phone_number: String,
    pub password: String,
}

pub fn load_config() -> Result<ConfigData, Box<dyn Error>> {
    let api_id: i32 = env::var("API_ID")?.parse()?;
    let api_hash: String = env::var("API_HASH")?;
    let phone_number: String = env::var("PHONE_NUMBER")?;
    let password: String = env::var("PASSWORD")?;

    Ok(ConfigData {
        api_id,
        api_hash,
        phone_number,
        password,
    })
}

pub async fn load_or_create_session(session_file: &str) -> Result<Session, Box<dyn Error>> {
    if let Ok(data) = tokfs::read(session_file).await {
        Ok(Session::load(&data)?)
    } else {
        Ok(Session::new())
    }
}

pub async fn save_session(session_file: &str, session_data: Vec<u8>) -> Result<(), Box<dyn Error>> {
    tokfs::write(session_file, session_data).await?;
    Ok(())
}
