use std::env;
use std::error::Error;
use std::fs;
use std::path::Path;

use grammers_session::Session;

#[derive(Debug)]
pub struct ConfigData {
    pub api_id: i32,
    pub api_hash: String,
    pub phone_number: String,
    pub password: String,
}

pub fn load_tg_client_config() -> Result<ConfigData, Box<dyn Error>> {
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
