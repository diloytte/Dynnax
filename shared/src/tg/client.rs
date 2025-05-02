use std::{
    error::Error,
    io::{self},
};

use crate::tg::config::{load_or_create_session, load_tg_client_config, save_session};
use grammers_client::{Client, Config, SignInError};

pub enum ClientType {
    Trader,
    Informer,
}

pub async fn connect_client(
    session_path: &str,
    client_type: ClientType,
) -> Result<Client, Box<dyn Error>> {
    let config = load_tg_client_config(client_type)?;

    let session = load_or_create_session(session_path).await?;

    let client = Client::connect(Config {
        session,
        api_id: config.api_id,
        api_hash: config.api_hash,
        params: Default::default(),
    })
    .await?;

    if !client.is_authorized().await? {
        let token = client.request_login_code(&config.phone_number).await?;

        println!("Enter the OTP code: ");
        let mut code = String::new();
        io::stdin().read_line(&mut code)?;
        let code = code.trim();

        match client.sign_in(&token, code).await {
            Ok(_) => println!("Logged in successfully!"),
            Err(SignInError::PasswordRequired(password_token)) => {
                client
                    .check_password(password_token, &config.password)
                    .await?;
            }
            Err(e) => return Err(e.into()),
        }
    }

    let session_data = client.session().save();
    save_session(session_path, session_data)?;

    println!("Connected to Telegram via {}!",client.get_me().await.unwrap().full_name());
    Ok(client)
}
