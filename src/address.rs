use std::fmt;
use rust_base58::ToBase58;
use crypto::digest::Digest;
use crypto::sha2::Sha256;
use crypto::ripemd160::Ripemd160; 
// use bech32;

#[derive(Clone, PartialEq, Eq, Hash)]
pub enum Payload {
    PubkeyHash(Vec<u8>),
    // ScriptHash(Vec<u8>),
    // WitnessProgram {
    //     version: bech32::u5,
    //     program: Vec<u8>,
    // },
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub enum Network {
    Mainnet,
    // Testnet
}

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct BitcoinAddress {
    network : Network,
    payload : Payload
}

impl fmt::Display for BitcoinAddress {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.payload {
            Payload::PubkeyHash(ref payload) => {
                let mut address = Vec::new();
                let prefix = match self.network {
                    Network::Mainnet => vec![0x00],
                    // Network::Testnet => vec![0x6F]
                };
                address.extend(prefix);
                address.extend(payload);
                let checksum = double_sha256(&address);
                address.extend(checksum[..4].iter().cloned());
                write!(f,"{}", address.to_base58())
            }
            // Payload::ScriptHash(ref payload) => {
            //     let mut address = Vec::new();
            //     let prefix = match self.network {
            //         Network::Mainnet => vec![0x05],
            //         Network::Testnet => vec![0xC4]
            //     };
            //     address.extend(prefix);
            //     address.extend(payload);
            //     let checksum = double_sha256(&address);
            //     address.extend(checksum[..4].iter().cloned());
            //     write!(f,"{}", address.to_base58())
            // }
            // Payload::WitnessProgram {
            //     version: v,
            //     program: ref p,
            // } => {
            //     let prefix = match self.network {
            //         Network::Mainnet => "bc",
            //         Network::Testnet => "tb"
            //     };
            //     let mut bech32_writer = bech32::Bech32Writer::new(prefix, f)?;
            //     bech32::WriteBase32::write_u5(&mut bech32_writer, v)?;
            //     bech32::ToBase32::write_base32(p, &mut bech32_writer)
            // }
        }
	}
}

impl BitcoinAddress {

    pub fn p2pkh(public_key :&[u8], network :Network) -> BitcoinAddress{
        if public_key.len() != 33 {
            panic!("Public key must be 33 bytes(compressed) length");
        }
        let hash = hash160(&public_key);
        BitcoinAddress {
            network : network,
            payload : Payload::PubkeyHash(hash)
        }
    }

    // pub fn p2sh(script: &[u8], network :Network) -> BitcoinAddress {
    //     if script.len() == 0 {
    //         panic!("Script must be at least 1 opcode");
    //     }
    //     let hash = hash160(&script);
    //     BitcoinAddress {
    //         network : network,
    //         payload : Payload::ScriptHash(hash)
    //     }
    // }

    // pub fn p2wpkh(public_key : &[u8], network :Network) -> BitcoinAddress {
    //     if public_key.len() != 33 {
    //         panic!("Public key must be 33 bytes(compressed) length")
    //     }
    //     let mut script = vec![0x00, 0x14]; //OP_0 + 20-byte push
    //     script.extend(public_key.to_vec());
    //     let hscript = hash160(&script);
    //     BitcoinAddress {
    //         network : network,
    //         payload : Payload::WitnessProgram {
    //             version: bech32::u5::try_from_u8(0).unwrap(),
    //             program : hscript
    //         }
    //     }
    // }

    // pub fn p2wsh(script: &[u8], network :Network) -> BitcoinAddress{
    //     if script.len() == 0 {
    //         panic!("Script must be at least 1 opcode");
    //     }
    //     let mut mscript = vec![0x00, 0x20]; //OP_0 + 32-byte push 
    //     mscript.extend(script.to_vec());
    //     let hscript = sha256(&script); 
    //     BitcoinAddress {
    //         network : network,
    //         payload : Payload::WitnessProgram {
    //             version: bech32::u5::try_from_u8(0).unwrap(),
    //             program : hscript
    //         }
    //     }
    // }
    
}

//return double sha256 as a byte array
fn double_sha256(bytes : &[u8]) -> Vec<u8> {
    let mut hasher = Sha256::new();
    let mut hash = vec![0; hasher.output_bytes()];
    hasher.input(&bytes);
    hasher.result(&mut hash);
    hasher.reset();
    hasher.input(&hash);
    hasher.result(&mut hash);
    return hash;
}

fn hash160(bytes : &[u8]) -> Vec<u8> {
    let mut res = sha256(&bytes);
    res = ripemd160(&res);
    return res;
}

//return sha256 as a byte array
fn sha256(bytes : &[u8]) -> Vec<u8> {
    let mut hasher = Sha256::new();
    let mut hash = vec![0; hasher.output_bytes()];
    hasher.input(&bytes); 
    hasher.result(&mut hash);
    return hash;
}

//return ripemd as a byte array
fn ripemd160(bytes : &[u8]) -> Vec<u8> {
    let mut ripemder = Ripemd160::new();
    let mut hash = vec![0; ripemder.output_bytes()];
    ripemder.input(&bytes); 
    ripemder.result(&mut hash);
    return hash;
}






/* TODO
#[cfg(test)]
mod tests {
    #[test]
    fn exploration() {
        assert_eq!(2 + 2, 4);
    }
}
*/