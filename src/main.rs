extern crate secp256k1;
extern crate crypto;
extern crate rand;
extern crate rust_base58;
extern crate bech32;

use secp256k1::Secp256k1;
use rand::rngs::OsRng;
use std::collections::HashMap;

mod address;
mod receiver;

use serde::Deserialize;

#[tokio::main]
fn main() {
    let client = redis::Client::open("redis://127.0.0.1/")?;
    let mut con = client.get_connection()?;
    let receiver_handle = std::thread::spawn(move || {
        receiver::receive();
    });
        
    loop{
        let mut key_pairs: HashMap<String, secp256k1::SecretKey> = HashMap::new();
        let mut address_string = String::with_capacity(20 * 35); // 20 address with 34 length + '|'

        for _ in 0..20 {
            //generate private and public keys
            let secp256k1 = Secp256k1::new();
            let mut rng = OsRng::new().expect("OsRng");
            let (_private_key, public_key) = secp256k1.generate_keypair(&mut rng);
            let serialized_public_key = public_key.serialize();
            let _address = address::BitcoinAddress::p2pkh(&serialized_public_key, address::Network::Mainnet);
        }
    }
    Ok(())
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


