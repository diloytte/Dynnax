use crate::tg::config::{load_config, load_or_create_session, save_session};
use crate::{Client, Config};
use crate::SignInError;

pub async fn connect_client() -> Result<Client, Box<dyn std::error::Error>> {
    let config = load_config()?; 

    let session_file = "session.session";
    let session = load_or_create_session(session_file).await?;

    let client = Client::connect(Config {
        session,
        api_id: config.api_id,
        api_hash: config.api_hash,
        params: Default::default(),
    })
    .await?;

    if !client.is_authorized().await? {
        let token = client.request_login_code(&config.phone_number).await?;

        println!("Enter the OTP code:");
        let mut code = String::new();
        std::io::stdin().read_line(&mut code)?;
        let code = code.trim();

        match client.sign_in(&token, code).await {
            Ok(_) => println!("Logged in successfully!"),
            Err(SignInError::PasswordRequired(password_token)) => {
                client.check_password(password_token, &config.password).await?;
            }
            Err(e) => return Err(e.into()),
        }
    }

    let session_data = client.session().save();
    save_session(session_file, session_data).await?;

    println!("Connected to Telegram!");

    Ok(client)
}
