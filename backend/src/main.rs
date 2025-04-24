mod constants;
mod db;
mod types;
mod pf;
mod routes;
mod state;
mod tg;
mod utils;
mod twitter_regex;
mod sniper;

use grammers_client::types::Chat;
use dashmap::DashMap;
use db::connect::connect;
use dotenv::dotenv;

use shared::{tg::{client::connect_client, dialog::{find_dialog::find_dialog_chat_by_id, get_dialogs::get_dialogs_data}}};
use tower_http::cors::{Any, CorsLayer};
use std::{env, sync::Arc};
use tg::next_update_loop::main_tg_loop;
use utils::load_snipe_configurations;

use axum::{http::Method, Extension, Router};
use routes::{fallback, routes};
use state::AppState;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    let port: &str = if cfg!(feature = "production") {
        println!("Running PRODUCTION backend build");
        "8001"
    } else {
        println!("Running DEVELOPMENT backend build");
        "8000"
    };
    

    let listener = TcpListener::bind(format!("{}:{}", "localhost", port))
        .await
        .unwrap();

    let db_url = env::var("DATABASE_URL")?;

    let redacted_self_bot_father_dialog_id: i64 =
        env::var("REDACTED_SELF_BOT_FATHER_DIALOG_ID")?.parse()?;

    let db = connect(db_url).await.unwrap();

    sqlx::migrate!("./migrations").run(&db).await.unwrap();

    let client = connect_client("./backend/session.session").await?;

    let dialogs_data = get_dialogs_data(&client).await?;

    let dialogs_dashmap = DashMap::new();

    let sniper_trenches_chat_id:i64 = env::var("SNIPER_TRENCHES_CHAT_ID")?.parse()?;
    
    let sniper_trenches_chat:Chat = find_dialog_chat_by_id(&client, sniper_trenches_chat_id).await.unwrap();

    for dialog in dialogs_data {
        dialogs_dashmap.insert(dialog.id, (dialog.name, dialog.dialog_type));
    }

    let pf_api_key = if cfg!(feature = "production") {
        env::var("PUMPFUN_PORTAL_API_KEY")?
    } else {
        env::var("PUMPFUN_PORTAL_API_KEY_DEV")?
    };

    let pump_portal_url: &str = "https://pumpportal.fun/api/trade?api-key=";

    let pf_api_url = format!("{}{}",pump_portal_url,pf_api_key);

    let state = AppState {
        db,
        all_dialogs: dialogs_dashmap,
        snipe_targets: DashMap::default(),
        twitter_snipe_targets: DashMap::default(),
        tg_client: Some(client.clone()),
        redacted_custom_bot_id: redacted_self_bot_father_dialog_id,
        sniper_trenches_chat,
        pf_api_url,
        priority_fee_multiplier:1
    };

    let shared_state = Arc::new(state);

    load_snipe_configurations(&shared_state).await.unwrap();

    let cors = CorsLayer::new()
    .allow_origin(Any)
    .allow_methods([Method::GET, Method::POST, Method::PUT, Method::DELETE,Method::PATCH])
    .allow_headers([
        "Content-Type".parse().unwrap(),
        "Authorization".parse().unwrap(),
        "Access-Control-Allow-Origin".parse().unwrap(),
    ]);

    tokio::spawn(main_tg_loop(
        client.clone(),
        shared_state.clone(),
    ));

    tokio::spawn(async move {
        let router = Router::new()
            .nest("/api/v1", routes())
            .layer(cors)
            .layer(Extension(shared_state))
            .fallback(fallback);

        axum::serve(listener, router).await.unwrap();
    });

    loop {
        
    }
}
