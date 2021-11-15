use chrono::{Utc};
use std::io::{stdin, stdout, Result, Write};

fn main() -> Result<()> {
    let mut buffer = String::new();
    stdin().read_line(&mut buffer)?;
    
    let stripped = buffer.trim();
    let now = Utc::now().to_rfc2822();
    let stamp = format!("{} - [{}]\n", stripped, now);
    
    stdout().write_all(stamp.as_bytes())?;

    Ok(())
}
