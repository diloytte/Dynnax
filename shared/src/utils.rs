use std::{fs::File, io::{self, BufReader}, path::PathBuf, process::Command};
use rodio::{Decoder, OutputStream, Source};

use crate::types::{Browser, SoundError};


#[macro_export]
macro_rules! json_error {
    ($message:expr) => {
        serde_json::json!({ "error": $message }).to_string()
    };
}

pub fn open_browser(browser: Browser, url: &String) -> io::Result<()> {
    #[cfg(not(feature = "remote"))]
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

pub fn play_buy_notif(){
    #[cfg(not(feature = "remote"))]
    tokio::spawn(async{
        match play_sound(){
            Ok(_) => {},
            Err(error) => {
                println!("{}",error)
            },
        }
    });
} 

fn play_sound()->Result<(),SoundError> {
    //TODO: Good for now, but can be done better. Either saving something to in memory state, or using Sink (check rodio docs).
    let (_stream,stream_handle) = OutputStream::try_default()?;
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push("src/assets/buy.wav");
    let file = BufReader::new(File::open(path)?);
    let source = Decoder::new(file)?;
    let _ = stream_handle.play_raw(source.convert_samples());
    std::thread::sleep(std::time::Duration::from_secs(2));
    Ok(())
}