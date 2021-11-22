use chrono::{Utc, DateTime};
use std::time::{SystemTime};
use std::io::{stdin, stdout, Result, Write};
use sha2::{Sha256, Digest};

fn main() -> Result<()> {
    let mut buffer = String::new();
    stdin().read_line(&mut buffer)?;
    
    let stripped = buffer.trim();
    let mut hasher = Sha256::new();
    hasher.update(stripped);
    let hashresult = hasher.finalize();
    
    let now = SystemTime::now();
    let now_chrono: DateTime<Utc> = DateTime::from(now);

    println!("Input = {}", stripped);
    let stamp = format!("{:02x} - [{:?}]\n", hashresult ,now_chrono.to_rfc2822());

    stdout().write_all(stamp.as_bytes())?;

    Ok(())
}
