use reqwest::Client;
use std::{sync::Arc, time::Duration};
use tokio::{task, time, time::Instant};

pub async fn start_keep_alive(api_key: String) {
    let pf_api_url = format!("https://pumpportal.fun/api/trade?api-key={}", api_key);
    let client = Arc::new(Client::new());

    send_dummy_request(&client, &pf_api_url).await;

    let client = Arc::clone(&client);
    let url = pf_api_url.clone();

    task::spawn(async move {
        let mut interval = time::interval(Duration::from_secs(70));

        loop {
            interval.tick().await;

            let client = Arc::clone(&client);
            let url = url.clone();

            task::spawn(async move {
                send_dummy_request(&client, &url).await;
            });
        }
    });

    println!("Started Pumpportal keep-alive task.");
}

async fn send_dummy_request(client: &Client, url: &str) {
    let start = Instant::now();
    let response = client
        .post(url)
        .send()
        .await;

    match response {
        Ok(res) => {
            let ms = start.elapsed().as_millis();
            println!("Keep-alive ping: {} ms (status: {})", ms, res.status());
        }
        Err(err) => {
            eprintln!("Ping failed: {:?}", err);
        }
    }
}
