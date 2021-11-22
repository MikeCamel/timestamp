use chrono::{Utc, DateTime};
use std::time::{SystemTime};
use std::io::{stdin, stdout, Result, Write};
use sha2::{Sha256, Digest};
use openssl::sign::Signer;
use openssl::hash::MessageDigest;
use openssl::pkey::PKey;
use openssl::pkey::Private;
use openssl::rsa::*;
use std::str;

fn main() -> Result<()> {
    //TODO - multiple runs (only print public key once?)
    //TODO - prettify output (inc. signature?)
    //TODO - stdout.write_all for all output
    let keypair = generate_keys();
    let keypair = PKey::from_rsa(keypair).unwrap();
    let pubkey: Vec<u8> = keypair.public_key_to_pem().unwrap();

    println!("We'll be operating with the following public key:");
    println!("{:?}", str::from_utf8(pubkey.as_slice()).unwrap());

    let mut buffer = String::new();
    stdin().read_line(&mut buffer)?;
    
    let stripped = buffer.trim();
    let mut hasher = Sha256::new();
    hasher.update(stripped);
    let hashresult = hasher.finalize();

    let mut signer = Signer::new(MessageDigest::sha256(), &keypair).unwrap();
    signer.update(&hashresult)?;

    let now = SystemTime::now();
    let now_chrono: DateTime<Utc> = DateTime::from(now);
    

    signer.update(&now_chrono.to_string().as_bytes())?;
    let signature = signer.sign_to_vec().unwrap();

    let input = format!("Input = {}\n", stripped);
    let hashout = format!("Hash = {:02x}\n", hashresult);
    let stamp = format!("[{:?}]\n", now_chrono.to_rfc2822());
    let signature_str = format!("{:?}\n", signature);
    
    stdout().write_all(input.as_bytes())?;
    stdout().write_all(hashout.as_bytes())?;
    stdout().write_all(stamp.as_bytes())?;
    stdout().write_all(signature_str.as_bytes())?;
    Ok(())
}

fn generate_keys() -> Rsa<Private> {
    let key_length = 2048;
    println!("Generating key pair");
    let rsa = Rsa::generate(key_length).unwrap();
    rsa
}
