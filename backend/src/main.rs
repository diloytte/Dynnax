mod constants;
mod db;
mod middlewares;
mod pf;
mod routes;
mod sniper;
mod state;
mod tg;
mod types;
mod utils;

use dashmap::DashMap;
use dotenv::dotenv;
use grammers_client::types::Chat;

use pf::start_keep_alive;
use reqwest::Client;
use shared::{
    db::connect::connect,
    tg::{
        client::connect_client,
        dialog::{find_dialog::find_dialog_chat_by_id, get_dialogs::get_dialogs_data},
    },
    utils::build_cors_layer,
};
use std::{env, sync::Arc};
use tg::next_update_loop::main_tg_loop;
use utils::load_snipe_configurations;

use axum::{Extension, Router};
use routes::{fallback, routes};
use state::AppState;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();
    let db_url = env::var("DATABASE_URL")?;
    let db = connect(db_url).await.unwrap();
    sqlx::migrate!("./migrations").run(&db).await.unwrap();

    let client = connect_client(
        "./backend/session.session",
        shared::tg::client::ClientType::Trader,
    )
    .await?;

    let client_informer = connect_client(
        "./backend/session_informer.session",
        shared::tg::client::ClientType::Informer,
    )
    .await?;

    let dialogs_dashmap = DashMap::new();
    let dialogs_data = get_dialogs_data(&client).await?;
    for dialog in dialogs_data {
        dialogs_dashmap.insert(dialog.id, (dialog.name, dialog.dialog_type));
    }

    let pf_api_key = if cfg!(feature = "production") {
        env::var("PUMPFUN_PORTAL_API_KEY")?
    } else {
        env::var("PUMPFUN_PORTAL_API_KEY_DEV")?
    };

    let pump_portal_url: &str = "https://pumpportal.fun/api/trade?api-key=";
    let pf_api_url = format!("{}{}", pump_portal_url, pf_api_key);

    let redacted_self_bot_father_dialog_id: i64 =
        env::var("REDACTED_SELF_BOT_FATHER_DIALOG_ID")?.parse()?;

    let sniper_trenches_chat_id: i64 = env::var("SNIPER_TRENCHES_CHAT_ID")?.parse()?;
    let sniper_trenches_chat: Chat = find_dialog_chat_by_id(&client, sniper_trenches_chat_id)
        .await
        .unwrap();

    let trojan_bot_chat_id: i64 = env::var("TROJAN_DIALOG_ID")?.parse()?;
    let trojan_bot_chat = find_dialog_chat_by_id(&client, trojan_bot_chat_id)
        .await
        .unwrap();

    let dynnax_api_key = env::var("API_KEY")?;

    let state = AppState {
        request_client: Client::new(),
        db,
        all_dialogs: dialogs_dashmap,
        tg_client_informer: client_informer,
        snipe_targets: DashMap::default(),
        twitter_snipe_targets: DashMap::default(),
        tg_client: client.clone(),
        redacted_custom_bot_id: redacted_self_bot_father_dialog_id,
        redacted_bot_chat: find_dialog_chat_by_id(&client, redacted_self_bot_father_dialog_id)
            .await
            .unwrap(),
        sniper_trenches_chat: Arc::new(sniper_trenches_chat),
        pf_api_url: pf_api_url.clone(),
        priority_fee_multiplier: 1,
        trojan_bot_chat: Arc::new(trojan_bot_chat),
        dynnax_api_key,
    };

    let shared_state = Arc::new(state);

    load_snipe_configurations(&shared_state).await.unwrap();

    tokio::spawn(main_tg_loop(client.clone(), shared_state.clone()));

    start_keep_alive(pf_api_key.clone()).await;

    let router = Router::new()
        .nest("/api/v1", routes())
        .layer(build_cors_layer())
        .layer(Extension(shared_state))
        .fallback(fallback);

    let port = if cfg!(feature = "production") {
        println!("Running PRODUCTION backend build");
        "8001"
    } else {
        println!("Running DEVELOPMENT backend build");
        "8000"
    };

    let bind_address = if cfg!(any(feature = "remote", feature = "production")) {
        "0.0.0.0"
    } else {
        "localhost"
    };

    println!("Listening on: {}:{}", bind_address, port);

    let listener = TcpListener::bind(format!("{}:{}", bind_address, port))
        .await
        .unwrap();

    axum::serve(listener, router).await.unwrap();

    Ok(())
}
