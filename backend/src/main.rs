mod constants;
mod db;
mod models;
mod pf;
mod routes;
mod state;
mod tg;
mod utils;

use dashmap::DashMap;
use db::connect::connect;
use dotenv::dotenv;
use std::{env, sync::Arc};
use tg::{
    client::connect_client, dialog::get_dialogs::get_dialogs, next_update_loop::main_tg_loop,
};
use utils::{load_snipe_configurations, play_buy_notif};

use axum::{Extension, Router};
use routes::{fallback, routes};
use state::AppState;
use tokio::net::TcpListener;

use grammers_client::{Client, Config, SignInError};
use grammers_session::Session;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    let port = if cfg!(feature = "production") {
        "8001" // production port
    } else {
        "8000" // development port
    };


    let listener = TcpListener::bind(format!("{}:{}", "localhost", port))
        .await
        .unwrap();

    let db_url = env::var("DATABASE_URL")?;

    let redacted_self_bot_father_dialog_id: i64 =
        env::var("REDACTED_SELF_BOT_FATHER_DIALOG_ID")?.parse()?;

    let db = connect(db_url).await.unwrap();

    sqlx::migrate!("./migrations").run(&db).await.unwrap();

    let client = connect_client().await?;

    let dialogs = get_dialogs(&client).await?;

    let dialogs_dashmap = DashMap::new();

    for dialog in dialogs {
        dialogs_dashmap.insert(dialog.id, (dialog.name, dialog.dialog_type));
    }

    let state = AppState {
        db,
        all_dialogs: dialogs_dashmap,
        snipe_targets: DashMap::default(),
        twitter_snipe_targets: DashMap::default(),
        tg_client: Some(client.clone()),
        redacted_custom_bot_id: redacted_self_bot_father_dialog_id,
    };

    let shared_state = Arc::new(state);

    load_snipe_configurations(&shared_state).await.unwrap();

    // This part will only run in production mode
    #[cfg(not(feature = "production"))]
    {
        println!("Running a development build of Dynnax application.");

        // Do not run in dev mode
        loop {
            
        }
    }
    
    let pf_api_key = if cfg!(feature = "production") {
        env::var("PUMPFUN_PORTAL_API_KEY")?
    } else {
        env::var("PUMPFUN_PORTAL_API_KEY_DEV")?
    };


    println!("Running a production build of Dynnax application.");

    tokio::spawn(main_tg_loop(
        client.clone(),
        shared_state.clone(),
        pf_api_key.clone(),
    ));

    tokio::spawn(async move {
        let router = Router::new()
            .nest("/api/v1", routes())
            .layer(Extension(shared_state.clone()))
            .fallback(fallback);

        axum::serve(listener, router).await.unwrap();
    });

    loop {
        
    }

    Ok(())
}
