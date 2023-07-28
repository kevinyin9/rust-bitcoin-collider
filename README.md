# Bitcoin Collider in Rust

The idea is from https://lbc.cryptoguru.org/about

> The "Large Bitcoin Collider" (LBC - a homage to LHC) is a distributed effort to find at least one collision of private Bitcoin keys by creating addresses to private keys in a continuous 2^160 range. These are checked against the list of known BTC addresses with funds on them. In the rare event of a collision, the funds on the address in question would become accessible to the collision finder.

> *Gasp* That's Illegal! Racist! Impossible!

> It's neither of these. For the history and reasons why this project started, see [this topic on bitcointalk](https://bitcointalk.org/index.php?topic=1555043.0). It is not illegal to search for colliding private keys. It may be illegal - depending on the jurisdiction you are in - to actually claim possession of funds found that way. It is also not impossible and actually the pool has already found several private keys - see [pool trophies](https://lbc.cryptoguru.org/trophies).

> Why doing this?
> Because current consensus is "that's impossible" and that is a gauntlet thrown down. It is a technical challenge and in mankind history, many things deemed impossible later turned out to be perfectly possible. This project is the practice part of the theory behind Bitcoin encryption and protection of funds. See our take at the [theory](https://lbc.cryptoguru.org/man/theory) behind all this.

## Usage
```
cargo run --release
```

## TODO List

* Add P2PK

## [Here is four main cryptography that Bitcoin used](https://unchained.com/blog/bitcoin-address-types-compared/):
This repository implements P2PKH, P2SH, P2WPKH, and P2WSH. However, given that P2PKH addresses hold the majority of Bitcoin, the current focus of this repository is solely on P2PKH.

| Type	| First Seen | BTC Supply| 	Use	| Encoding| 	Prefix| 	Characters| 
| --- | --- | --- | --- |--- |--- |--- |
| P2PK	| Jan 2009	|9% (1.7M)	|Obsolete	|		
| P2PKH	| Jan 2009	|43% (8.3M)	|Decreasing|	Base58|	1	|26 – 34|
| P2MS	| Jan 2012	|Negligible	|Obsolete		|	
| P2SH	| Apr 2012	|24% (4.6M)	|Decreasing|	Base58	|3	|34|
| P2WPKH	| Aug 2017|	20% (3.8M)	|Increasing	|Bech32	|bc1q	|42|
| P2WSH	| Aug 2017	|4% (0.8M)	|Increasing	|Bech32	|bc1q	|62|
| P2TR	| Nov 2021	|0.1% (0.02M)|	Increasing	|Bech32m	|bc1p	|62|

### P2PK

Pay-to-Public-Key (P2PK) is the original method of receiving bitcoin, and it does not involve an address. Instead, as the name suggests, bitcoin is paid directly to an exposed public key. The first ever bitcoin transaction from one person to another used P2PK, when Satoshi Nakamoto sent coins to Hal Finney in Block 170.

P2PK is no longer used because it is a more expensive, less private, and less secure way of receiving bitcoin than subsequent methods.

First seen:	Block 0 | January 3, 2009

Example recipient: 04678afdb0fe5548271967f…384df7ba0b8d578a4c702b6bf11d5

Current supply: ~ 1.7M BTC or 9%

Status:	Obsolete

### P2PKH

Pay-to-Public-Key-Hash (P2PKH) was available for use at bitcoin’s beginning, and it showed up on the blockchain for the first time less than two weeks after the genesis block. P2PKH makes several improvements upon P2PK, such as utilizing an address. As discussed in our earlier article, addresses contain a checksum that helps prevent typos and lost bitcoin.

P2PKH addresses are typically 34 or 33 characters in length (but could theoretically be as short as 26 characters), and they are encoded in Base58 format. They begin with a prefix of 1 and are currently responsible for receiving and securing 43% of the mined bitcoin supply, more than any other address type.

Creating a P2PKH address involves putting a single public key through hash functions SHA-256 and RIPEMD-160. This shortens the amount of data, which in turn helps save block space and transaction fees for the user. It also introduces further resistance to reverse-engineering the private key beyond the already believed-to-be-unbreakable secp256k1 elliptic curve.

First seen: Block 728 | January 16, 2009

Example recipient:	12higDjoCCNXSA95xZMWUdPvXNmkAduhWv

Current supply:	~ 8.3M BTC or 43%

Status: Decreasing popularity

### P2SH

Pay-to-Script-Hash (P2SH) was introduced to bitcoin as a soft fork in accordance with BIP 16 on April 1, 2012. Like most forks, the story behind it is fascinating. P2SH shares a lot in common with P2PKH. The main difference is that the address is created by hashing a redeem script instead of hashing a single public key. 

A redeem script can be thought of as coded instructions specifying how bitcoin received to the P2SH address can be spent in the future. There could be a wide range of possibilities, including multiple different public keys. The receiver, not the sender, determines the script details, and the spending instructions are not exposed publicly until bitcoin is spent out of the address.

While advanced users can construct complex scripts, the most common uses for P2SH have been to create Nested SegWit addresses (covered below) and multisig wallets. For example, a script can include three public keys and specify that signatures from any two of the corresponding private keys can spend the bitcoin. This would create a 2-of-3 multisig address.

P2SH addresses are exactly 34 characters in length, and they begin with a prefix of 3, as specified by BIP 13. Before the soft fork on April 1st, a handful of transactions experimented with this alternative prefix, the first of which is found in Block 170,052.

First seen:	Block 174,717 | April 7, 2012

Example recipient:	342ftSRCvFHfCeFFBuz4xwbeqnDw6BGUey

Current supply:	~ 4.6M BTC or 24%

Status:	Decreasing popularity

### P2WPKH

Pay-to-Witness-Public-Key-Hash (P2WPKH) is the first of two address types introduced to bitcoin upon the SegWit soft fork in August 2017. The story behind this extremely important and particularly contentious soft fork is documented in a book called The Blocksize War, written by Jonathan Bier.

P2WPKH is the SegWit variant of P2PKH, which at a basic level, means that choosing this address type rather than older P2PKH addresses will help you save money on transaction fees when moving your bitcoin around.

SegWit addresses look quite different from the older address types because, per BIP 173, they use Bech32 encoding instead of Base58. Most notably, there are no capital letters in Bech32. P2WPKH addresses can be identified by a prefix of bc1q and a character length of exactly 42.

First seen:	Block 481,824 | August 23, 2017

Example recipient: bc1q34aq5drpuwy3wgl9lhup9892qp6svr8ldzyy7c

Current supply:	~ 3.8M BTC or 20%

Status:	Increasing popularity
