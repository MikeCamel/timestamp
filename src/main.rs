use chrono::{Utc, DateTime};
use std::time::{SystemTime};
use std::io::{stdin, stdout, Result, Write};
use sha2::{Sha256, Digest};
use p256::{ecdsa::{SigningKey, signature::Signer, VerifyingKey}};
use rand_core::OsRng;

fn main() -> Result<()> {
    //TODO - multiple runs (only print public key once?)
    //TODO - prettify output (inc. signature?)
    //TODO - sntp support
    let signing_key = SigningKey::random(&mut OsRng); 
    let verify_key = VerifyingKey::from(&signing_key); 
    let initial_output = format!("We'll be operating with the following public key: {:?}", verify_key);
    stdout().write_all(initial_output.as_bytes())?;

    let mut buffer = String::new();
    stdin().read_line(&mut buffer)?;
    
    let stripped = buffer.trim();
    let mut hasher = Sha256::new();
    hasher.update(stripped);
    let hashresult = hasher.finalize();

    let now = SystemTime::now();
    let now_chrono: DateTime<Utc> = DateTime::from(now);
    //let message = hashresult+stampsvb
    let message = format!("{:02x} [{:?}]", hashresult, now_chrono.to_rfc2822());
    let signature = signing_key.sign(message.as_bytes());
    

    let input = format!("Input = {}\n", stripped);
    let messageout = format!("{}/n", message);
    let signature_str = format!("{:?}\n", signature);
    
    stdout().write_all(input.as_bytes())?;
    stdout().write_all(messageout.as_bytes())?;
    stdout().write_all(signature_str.as_bytes())?;
    Ok(())
}

