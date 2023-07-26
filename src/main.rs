extern crate secp256k1;
extern crate crypto;
extern crate rand;
extern crate rust_base58;
extern crate bech32;

use secp256k1::Secp256k1;
use rand::rngs::OsRng;
use std::env;
use hex::FromHex;

mod address;
mod query;

#[derive(Serialize, Deserialize)]
struct KeyPair {
    address: String,
    private_key: String,
}

fn main(){

    let args: Vec<String> = env::args().collect();
    let mut count = 1;
    let conn:;
    let thread_handle = std::thread::spawn(move || {
        let mut key_pairs: Vec<KeyPair> = Vec::new();
        
        loop {
            let key_pair_json: Option<String> = conn.rpop("key_pairs").unwrap();
        
            match key_pair_json {
                Some(json) => {
                    let key_pair: KeyPair = serde_json::from_str(&json).unwrap();
                    key_pairs.push(key_pair);
        
                    if key_pairs.len() >= 100 {
                        // let body = reqwest::get("https://blockchain.info/balance?active=\"\"")
                        //     .await?
                        //     .text()
                        //     .await?;

                        // println!("body = {:?}", body);
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
    
    loop{
        count += 1;
        println!("{}", count);
        //generate private and public keys
        let secp256k1 = Secp256k1::new();
        let mut rng = OsRng::new().expect("OsRng");
        let (_private_key, public_key) = secp256k1.generate_keypair(&mut rng);
        let serialized_public_key = public_key.serialize();

        // print!("Private Key : {}\n", _private_key);
        // print!("Public Key : {}\n", public_key);

        let _address = address::BitcoinAddress::p2pkh(&serialized_public_key, address::Network::Mainnet);
        let key_pair = KeyPair{
            address: _address.to_string(),
            private_key: _private_key.to_string(),
        };

        let key_pair_json = serde_json::to_strinng(&key_pair).unwrap();

        let _: () = conn.lpush("key_pairs", key_pair_json).unwrap();
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


