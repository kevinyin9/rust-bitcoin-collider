use reqwest;

use serde::Deserialize;

#[derive(Deserialize)]
struct 
fn get_balance() {
    let body = reqwest::get("https://blockchain.info/balance?active={}")
            .await?
            .text()
            .await?;

        println!("body = {:?}", body);
}