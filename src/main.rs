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

fn main(){

    let args: Vec<String> = env::args().collect();
    let mut count = 1;
    loop{
        count += 1;
        println!("{}", count);
        //generate private and public keys
        let secp256k1 = Secp256k1::new();
        let mut rng = OsRng::new().expect("OsRng");
        let (_secret_key, public_key) = secp256k1.generate_keypair(&mut rng);
        let serialized_public_key = public_key.serialize();

        // print!("Private Key : {}\n", _secret_key);
        // print!("Public Key : {}\n", public_key);

        let _address = address::BitcoinAddress::p2pkh(&serialized_public_key, address::Network::Mainnet);

        let body = reqwest::get("https://blockchain.info/balance?active={}")
            .await?
            .text()
            .await?;

        println!("body = {:?}", body);
        // print!("Address : {}\n", _address);
        // match args.len() {
        //     3 | 4 => {
        //         let network = if args.len() > 3 { &args[3] } else { "--mainnet" } ;
        //         match &network[..] {
        //             "--testnet" => {
        //                 exec_args_p2pkh_p2wpkh(&args, address::Network::Testnet, &serialized_public_key);
        //             },
        //             "--mainnet" => {
        //                 exec_args_p2pkh_p2wpkh(&args, address::Network::Mainnet, &serialized_public_key);
        //             }
        //             _ => {
        //                 eprintln!("error: invalid command");
        //                 help();
        //             }
        //         };
        //     },
        //     5 | 6 => {
        //         let script = parse_script(&args[4]);
        //         let network = if args.len() > 5 { &args[5] } else { "--mainnet" } ;
        //         match &network[..] {
        //             "--testnet" => {
        //                 exec_args_p2sh_p2wsh(&args, address::Network::Testnet, &script);
        //             },
        //             "--mainnet" => {
        //                 exec_args_p2sh_p2wsh(&args, address::Network::Mainnet, &script);
        //             }
        //             _ => {
        //                 eprintln!("error: invalid command");
        //                 help();
        //             }
        //         };
        //     },
        //     _ => {
        //         // show a help message
        //         eprintln!("error: invalid command");
        //         help()
        //     }
        // }
    }
}   

fn exec_args_p2pkh_p2wpkh(args: &Vec<String>, network: address::Network, serialized_public_key: &[u8]) {
    match &args[1][..] {
        "--type" => {
            match &args[2][..] {
                "p2pkh" => {
                    let _address = address::BitcoinAddress::p2pkh(&serialized_public_key, network);
                    print!("Address : {}\n", _address);
                },
                "p2wpkh" => {
                    let _address = address::BitcoinAddress::p2wpkh(&serialized_public_key, network);
                    print!("Address : {}\n", _address);
                }
                    _ => {
                    eprintln!("error: invalid command");
                    help();
                },  
            }
        }
        _ => {
            eprintln!("error: invalid command");
            help();
        },
    }
}

fn exec_args_p2sh_p2wsh(args: &Vec<String>, network: address::Network, script: &Vec<u8>) {
    match &args[3][..] {
        "--script" => {
            match &args[1][..] {
                "--type" => {
                    match &args[2][..] {
                        "p2sh" => {
                            let address = address::BitcoinAddress::p2sh(&script, network);
                            print!("Address : {}\n", address);
                        },
                        "p2wsh" => {
                            let address = address::BitcoinAddress::p2wsh(&script, network);
                            print!("Address : {}\n", address);
                        }
                        _ => {
                            eprintln!("error: invalid command");
                            help();
                        },  
                    }
                }
                _ => {
                    eprintln!("error: invalid command");
                    help();
                },
            }
        }
        _ => {
            eprintln!("error: invalid command");
            help();
        },
    }
}

fn parse_script(script :&String) -> Vec<u8>{
    let split_script = script.split(",");
    let vec_script: Vec<&str> = split_script.collect();
    let mut res : Vec<u8>= Vec::new(); 
    for el in vec_script {
        let elh = Vec::from_hex(el).unwrap();
        res.extend(elh);
    }
    return res;
}

fn help(){
    print!("usage: ./rust-bitcoin-address-generator --type <type> --script <script> [--mainnet | --testnet]\n");
    print!("where <type> can be [p2pkh,p2wpkh,p2sh,p2wsh] and <script> is an array of op_codes byte\n");
    print!("ex: ./rust-bitcoin-address-generator --type p2sh --script 00,14 --mainnet\n");
}


