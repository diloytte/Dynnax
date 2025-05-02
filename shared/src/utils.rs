#[cfg(not(feature = "remote"))]
use crate::types::{Browser, SoundError};
use reqwest::Method;
#[cfg(not(feature = "remote"))]
use rodio::{Decoder, OutputStream, Source};
use tower_http::cors::{Any, CorsLayer};
#[cfg(not(feature = "remote"))]
use std::{
    fs::File,
    io::{self, BufReader},
    path::PathBuf,
    process::Command,
};

#[macro_export]
macro_rules! json_error {
    ($message:expr) => {
        serde_json::json!({ "error": $message }).to_string()
    };
}

#[cfg(not(feature = "remote"))]
pub fn open_browser(browser: Browser, url: &str) -> io::Result<()> {
    match browser {
        Browser::Brave => {
            Command::new("brave-browser")
                .arg("--new-tab")
                .arg(url)
                .spawn()?;
        }
    }

    Ok(())
}

#[cfg(not(feature = "remote"))]
pub fn play_buy_notif() {
    tokio::spawn(async {
        match play_sound() {
            Ok(_) => {}
            Err(error) => {
                println!("{}", error)
            }
        }
    });
}

#[cfg(not(feature = "remote"))]
fn play_sound() -> Result<(), SoundError> {
    //TODO: Good for now, but can be done better. Either saving something to in memory state, or using Sink (check rodio docs).
    let (_stream, stream_handle) = OutputStream::try_default()?;
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push("src/assets/buy.wav");
    let file = BufReader::new(File::open(path)?);
    let source = Decoder::new(file)?;
    let _ = stream_handle.play_raw(source.convert_samples());
    std::thread::sleep(std::time::Duration::from_secs(2));
    Ok(())
}

pub fn build_cors_layer() -> CorsLayer {
    CorsLayer::new()
        .allow_origin(Any)
        .allow_methods([
            Method::GET,
            Method::POST,
            Method::PUT,
            Method::DELETE,
            Method::PATCH,
        ])
        .allow_headers([
            "Content-Type".parse().unwrap(),
            "Authorization".parse().unwrap(),
        ])
}