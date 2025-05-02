use reqwest::Client;
use serde_json::json;
use std::{sync::Arc, time::Duration};
use tokio::{task, time};

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

pub async fn send_dummy_request(client: &Client, url: &str) {
    #[cfg(feature = "performance_log")]
    let start = std::time::Instant::now();

    let body = json!({
        "action": "buy",
        "amount": 0.0000001,
        "denominatedInSol": "true",
        "mint": "8ncucXv6U6epZKHPbgaEBcEK399TpHGKCquSt4RnmX4f",
        "pool": "auto",
        "priorityFee": 0.00000000005,
        "slippage": 0.000001
    });

    let response = client.post(url).json(&body).send().await;

    match response {
        #[cfg_attr(not(feature = "performance_log"), allow(unused_variables))]
        Ok(res) => {
            #[cfg(feature = "performance_log")]
            {
                let status = res.status();
                let ms = start.elapsed().as_millis();
                println!("Keep-alive ping: {} ms (status: {})", ms, status);
            }
        }
        Err(err) => {
            eprintln!("Ping failed: {:?}", err);
        }
    }
}
