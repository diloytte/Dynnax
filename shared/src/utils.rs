use std::{fs::File, io::{self, BufReader}, process::Command};
use rodio::{Decoder, OutputStream, Source};

use crate::types::{Browser, SoundError};


#[macro_export]
macro_rules! json_error {
    ($message:expr) => {
        serde_json::json!({ "error": $message }).to_string()
    };
}

pub fn open_browser(browser: Browser, url: &String) -> io::Result<()> {
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
    let file = BufReader::new(File::open("src/assets/buy.wav")?);
    let source = Decoder::new(file)?;
    let _ = stream_handle.play_raw(source.convert_samples());
    std::thread::sleep(std::time::Duration::from_secs(2));
    Ok(())
}