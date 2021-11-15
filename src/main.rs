use chrono::{Duration, Utc, DateTime};
use std::time::{SystemTime,UNIX_EPOCH};
use std::io::{stdin, stdout, Result, Write};

fn main() -> Result<()> {
    let mut buffer = String::new();
    stdin().read_line(&mut buffer)?;
    
    let stripped = buffer.trim();
    //let now = Utc::now().to_rfc2822();
    let now = SystemTime::now();
    let now_chrono: DateTime<Utc> = DateTime::from(now);
    let stamp = format!("{} - [{:?}]\n", stripped,now_chrono.to_rfc2822());

    stdout().write_all(stamp.as_bytes())?;

    Ok(())
}
