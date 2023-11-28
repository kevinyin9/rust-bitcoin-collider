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

mod address;

use serde::Deserialize;

#[derive(Deserialize)]
struct Balance {
    final_balance: i64,
}

type BalanceMap = std::collections::HashMap<String, Balance>;


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // let client = redis::Client::open("redis://127.0.0.1/")?;
    // let mut con = client.get_connection()?;
    // let thread_handle = std::thread::spawn(move || {
    //     let mut key_pairs: Vec<KeyPair> = Vec::new();
        
        // loop {
            // let key_pair_json: Option<String> = conn.rpop("key_pairs").unwrap();
        
    //         match key_pair_json {
    //             Some(json) => {
    //                 let key_pair: KeyPair = serde_json::from_str(&json).unwrap();
    //                 key_pairs.push(key_pair);
        
    //                 if key_pairs.len() >= 100 {
    //                     // let body = reqwest::get("https://blockchain.info/balance?active=\"\"")
    //                     //     .await?
    //                     //     .text()
    //                     //     .await?;

    //                     // println!("body = {:?}", body);
    //                     key_pairs.clear();
    //                 }
    //             },
    //             None => {
    //                 // Sleep for a while before trying to get the next key pair.
    //                 std::thread::sleep(std::time::Duration::from_secs(1));
    //             }
    //         }
    //     }
    // });
    let mut counter: i64 = 0;   
    loop{
        let mut key_pairs: HashMap<String, secp256k1::SecretKey> = HashMap::new();
        let mut address_string = String::with_capacity(100 * 64);

        for _ in 0..19 {
            //generate private and public keys
            let secp256k1 = Secp256k1::new();
            let mut rng = OsRng::new().expect("OsRng");
            let (_private_key, public_key) = secp256k1.generate_keypair(&mut rng);
            let serialized_public_key = public_key.serialize();

            let _address = address::BitcoinAddress::p2pkh(&serialized_public_key, address::Network::Mainnet);

            key_pairs.insert(_address.clone().to_string(), _private_key.clone());

            address_string.push_str(&_address.to_string());
            address_string.push_str("|");
        }
        // For testing
        // let secp256k1 = Secp256k1::new();
        // let mut rng = OsRng::new().expect("OsRng");
        // let (_private_key, public_key) = secp256k1.generate_keypair(&mut rng);
        // key_pairs.insert("bc1p7d8n5y3zy3gqrd80huuu9926t9ctll9mla9vk6tvr98ccst9rp9s3uve3d".to_string(), _private_key.clone());
        // address_string.push_str("bc1p7d8n5y3zy3gqrd80huuu9926t9ctll9mla9vk6tvr98ccst9rp9s3uve3d|");
        
        counter += 20;
        let url = "https://blockchain.info/balance?active=".to_string() + &address_string.to_string();
        let response = query(&url).await?;
        
        
        for (address, balance) in response.iter() {
            if balance.final_balance != 0 {
                if let Some(key) = key_pairs.get(address) {
                    let data = format!("Address: {}, PrivateKey: {}, Final Balance: {}\n", address, key, balance.final_balance);
                    println!("{}", data);
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
        if counter % 10000 == 0 {
            println!("\rCurrent count = {}", counter);
        }
        
    }
    // Ok(())
}

async fn query(s: &String) -> Result<BalanceMap, Box<dyn std::error::Error>> {
    let response = reqwest::get(s).await?;
    let body = response.text().await?;
    
    match serde_json::from_str(&body) {
        Ok(balance_map) => Ok(balance_map),
        Err(e) => {
            eprintln!("body: {}", body);
            eprintln!("e: {}", e);
            Err(Box::new(e))
        }
    }
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


