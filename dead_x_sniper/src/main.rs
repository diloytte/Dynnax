use grammers_client::Client;
use shared::tg::client::connect_client;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client:Client = connect_client("./dead_x_sniper/session.session").await?;

    Ok(())
}
