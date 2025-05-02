use std::env;
use std::error::Error;
use std::fs;
use std::path::Path;

use grammers_session::Session;

use super::client::ClientType;

#[derive(Debug)]
pub struct ConfigData {
    pub api_id: i32,
    pub api_hash: String,
    pub phone_number: String,
    pub password: String,
}

pub fn load_tg_client_config(client_type: ClientType) -> Result<ConfigData, Box<dyn Error>> {
    let (api_id, api_hash, phone_number, password) = match client_type {
        ClientType::Trader => (
            env::var("API_ID")?.parse()?,
            env::var("API_HASH")?,
            env::var("PHONE_NUMBER")?,
            env::var("PASSWORD")?,
        ),
        ClientType::Informer => (
            env::var("INFORM_CLIENT_API_ID")?.parse()?,
            env::var("INFORM_CLIENT_API_HASH")?,
            env::var("INFORM_CLIENT_NUMBER")?,
            env::var("INFORM_CLIENT_PASSWORD")?,
        ),
    };

    Ok(ConfigData {
        api_id,
        api_hash,
        phone_number,
        password,
    })
}

pub async fn load_or_create_session(session_file: &str) -> Result<Session, Box<dyn Error>> {
    match fs::read(session_file) {
        Ok(data) => {
            let session = Session::load(&data)?;
            Ok(session)
        }
        Err(_) => Ok(Session::new()),
    }
}

pub fn save_session(session_file: &str, session_data: Vec<u8>) -> Result<(), Box<dyn Error>> {
    fs::write(Path::new(session_file), session_data)?;
    Ok(())
}
