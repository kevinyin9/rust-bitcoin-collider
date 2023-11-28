extern crate secp256k1;
extern crate crypto;
extern crate rand;
extern crate rust_base58;
extern crate bech32;

use secp256k1::Secp256k1;
use rand::rngs::OsRng;
use std::collections::HashMap;
use std::fs::OpenOptions;
use std::io::prelude::*;
use std::time::{Instant, Duration};
use std::sync::{Mutex, Arc};
use tokio::sync::watch;
// use tokio::time::Duration;

mod address;

use serde::Deserialize;

#[derive(Deserialize)]
struct Balance {
    final_balance: i64,
}

type BalanceMap = std::collections::HashMap<String, Balance>;


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let counter = Arc::new(Mutex::new(0 as i64));
    let key_pairs: Arc<Mutex<HashMap<String, secp256k1::SecretKey>>> = Arc::new(Mutex::new(HashMap::new()));
    let secp256k1 = Secp256k1::new();
    let (tx, mut rx) = watch::channel(String::new());
    for i in 0..3 {
        let (key_pairs, mut rx, counter) = (Arc::clone(&key_pairs), rx.clone(), Arc::clone(&counter));
        tokio::spawn(async move{
            loop {
                while rx.changed().await.is_ok() {
                    let url = rx.borrow_and_update().clone();
                    let response = query(&url).await.expect("fuck");

                    let key_pairs = key_pairs.lock().unwrap();
                    for (address, balance) in response.iter() {
                        // println!("{}: {}", address, balance.final_balance);
                        if balance.final_balance != 0 {
                            if let Some(key) = key_pairs.get(address) {
                                let data = format!("Address: {}, PrivateKey: {}, Final Balance: {}\n", address, key, balance.final_balance);
                                
                                let mut file = OpenOptions::new()
                                                    .create(true)
                                                    .append(true)
                                                    .open("result.txt")
                                                    .expect("cannot open file");
                                file.write(data.as_bytes())
                                    .expect("Unable to write data");
                            } else {
                                println!("Missing key for Address: {}, Final Balance: {}", address, balance.final_balance);
                            }
                        }
                    }
                    let mut counter = counter.lock().unwrap();
                    // let counter = Arc::try_unwrap(counter.into()).unwrap();
                    *counter += 50;
                    // println!("counter: {}", *counter);
                    if *counter % 10000 == 0 {
                        println!("\rCurrent count = {}", *counter);
                    }
                }
            }
        });
    }

    let key_pairs_clone = Arc::clone(&key_pairs);
    loop {
        // key_pairs.clear();
        let mut address_string = String::with_capacity(100 * 64);
        
        // let start = Instant::now();
        for _ in 0..50 {
            //generate private and public keys
            let mut rng = OsRng::new().expect("OsRng");
            let (_private_key, public_key) = secp256k1.generate_keypair(&mut rng);
            let serialized_public_key = public_key.serialize();

            let _address = address::BitcoinAddress::p2pkh(&serialized_public_key, address::Network::Mainnet);
            
            let mut key_pairs = key_pairs_clone.lock().unwrap();
            key_pairs.insert(_address.clone().to_string(), _private_key.clone());

            address_string.push_str(&_address.to_string());
            address_string.push_str("|");
        }
        // let duration = start.elapsed();
        // println!("Time taken to generate accounts: {:?}", duration);
        // println!("Rate: {} accounts per second", 20.0 / duration.as_secs_f64());

        // For testing
        // let secp256k1 = Secp256k1::new();
        // let mut rng = OsRng::new().expect("OsRng");
        // let (_private_key, public_key) = secp256k1.generate_keypair(&mut rng);
        // key_pairs.insert("bc1p7d8n5y3zy3gqrd80huuu9926t9ctll9mla9vk6tvr98ccst9rp9s3uve3d".to_string(), _private_key.clone());
        // address_string.push_str("bc1p7d8n5y3zy3gqrd80huuu9926t9ctll9mla9vk6tvr98ccst9rp9s3uve3d|");

        let url = "https://blockchain.info/balance?active=".to_string() + &address_string.to_string();
        // println!("{}", url);
        tx.send(url)?;
    }
    // Ok(())
}

async fn query(s: &String) -> Result<BalanceMap, Box<dyn std::error::Error>> {
    // println!("{}", s);
    const MAX_RETRIES: u32 = 5;
    const RETRY_DELAY: Duration = Duration::from_secs(5);
    for attempt in 1..=MAX_RETRIES {
        match reqwest::get(s).await {
            Ok(response) => {
                match response.text().await {
                    Ok(body) => {
                        match serde_json::from_str(&body) {
                            Ok(balance_map) => return Ok(balance_map),
                            Err(e) => {
                                eprintln!("Failed to parse response on attempt {}: {}", attempt, e);
                            }
                        }
                    }
                    Err(e) => {
                        eprintln!("Failed to get text from response on attempt {}: {}", attempt, e);
                    }
                }
            }
            Err(e) => {
                eprintln!("Request failed on attempt {}: {}", attempt, e);
            }
        }
    }
    Err(Box::new(std::io::Error::new(std::io::ErrorKind::Other, "Request failed after many times")))
}


// fn exec_args_p2pkh_p2wpkh(args: &Vec<String>, network: address::Network, serialized_public_key: &[u8]) {
//     let _address = address::BitcoinAddress::p2pkh(&serialized_public_key, network);
//     let _address = address::BitcoinAddress::p2wpkh(&serialized_public_key, network);
// }

// fn exec_args_p2sh_p2wsh(args: &Vec<String>, network: address::Network, script: &Vec<u8>) {
//     let address = address::BitcoinAddress::p2sh(&script, network);
//     let address = address::BitcoinAddress::p2wsh(&script, network);
// }

// fn parse_script(script :&String) -> Vec<u8>{
//     let split_script = script.split(",");
//     let vec_script: Vec<&str> = split_script.collect();
//     let mut res : Vec<u8>= Vec::new(); 
//     for el in vec_script {
//         let elh = Vec::from_hex(el).unwrap();
//         res.extend(elh);
//     }
//     return res;
// }

// fn help(){
//     print!("usage: ./rust-bitcoin-address-generator --type <type> --script <script> [--mainnet | --testnet]\n");
//     print!("where <type> can be [p2pkh,p2wpkh,p2sh,p2wsh] and <script> is an array of op_codes byte\n");
//     print!("ex: ./rust-bitcoin-address-generator --type p2sh --script 00,14 --mainnet\n");
// }


