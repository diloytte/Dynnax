use shared::tg::{client::connect_client, dialog::find_dialog::find_dialog_chat_by_id};
use grammers_client::Client;
use dotenv::dotenv;
use std::collections::HashMap;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();
    let client:Client = connect_client("./say_gm/session.session").await?;

    let mut gm_hashmap:HashMap<i64,&str> = HashMap::new();
    
    gm_hashmap.insert(1649642332,"/gm_lab");
    gm_hashmap.insert(1981115066,"/gm_printers");
    gm_hashmap.insert(2433957378,"gm");
    gm_hashmap.insert(2424480459,"gm");

    for (id,message) in gm_hashmap {
        let chat = find_dialog_chat_by_id(&client, id).await;
        match chat{
            Some(chat) =>{
                client.send_message(chat, message).await?;
            },
            None => {
                println!("Coud not find chat with ID: {}",id)
            },
        }
    }
    Ok(())
}