use reqwest;

use serde::Deserialize;

#[derive(Deserialize)]
struct Balance {
    final_balance: i64,
}

type BalanceMap = std::collections::HashMap<String, Balance>;

async fn receive() -> Result<(), Box<dyn std::error::Error>>{
    loop {
        let key_pair_json: Option<String> = conn.rpop("key_pairs").unwrap();
    
        match key_pair_json {
            Some(json) => {
                let key_pair: KeyPair = serde_json::from_str(&json).unwrap();
                key_pairs.insert(_address.clone().to_string(), _private_key.clone());

                address_string.push_str(&_address.to_string());
                address_string.push_str("|");
    
                if key_pairs.len() >= 20 {
                    query()
                    key_pairs.clear();
                }
            },
            None => {
                // Sleep for a while before trying to get the next key pair.
                std::thread::sleep(std::time::Duration::from_secs(1));
            }
        }
    }
});
    let url = "https://blockchain.info/balance?active=".to_string() + &address_string.to_string();
    let response = query(&url).await?;

    for (address, balance) in response.iter() {          
        if balance.final_balance != 0 {
            if let Some(private_key) = key_pairs.get(address) {
                println!("Address: {}, PrivateKey: {}, Final Balance: {}", address, private_key, balance.final_balance);
            }
        }
    }
}

async fn query(s: &String) -> Result<BalanceMap, Box<dyn std::error::Error>> {
    let response = reqwest::get(s).await?;
    let body = response.text().await?;
    let balance_map: BalanceMap = serde_json::from_str(&body)?;

    Ok(balance_map)
}