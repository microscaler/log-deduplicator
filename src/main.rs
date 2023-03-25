use sha2::{Digest, Sha256};
use std::collections::HashMap;
use std::io::{self, BufRead};
use std::time::{SystemTime};

// Define the time window for deduplication
const TIME_WINDOW: u64 = 5;

fn main() {
    let mut seen_signatures: HashMap<String, (u64, usize)> = HashMap::new();

    let mut hasher = Sha256::new();

    let stdin = io::stdin();
    let reader = stdin.lock();
    let mut lines = reader.lines();

    loop {
        let line = match lines.next() {
            Some(line) => line.expect("Failed to read line"),
            None => break,
        };

        let parts: Vec<&str> = line.split(" ").collect();
        let _timestamp = parts[0];
        let method = parts[1];
        let path = parts[2];
        let status = parts[3];
        let bytes_sent = parts[4];
        let referer = parts[5];
        let user_agent = parts[6..].join(" ").trim().clone().to_owned();
        let signature = format!(
            "{} {} {} {} {} {}",
            method, path, status, bytes_sent, referer, user_agent
        );

        hasher.update(signature.as_bytes());
        let signature_hash = hasher.finalize_reset();
        let signature_hash_hex = signature_hash.iter().map(|b| format!("{:02x}", b)).collect::<String>();

        let current_time = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .expect("Time went backwards")
            .as_secs();

        let (last_seen_time, count) = seen_signatures.entry(signature_hash_hex).or_insert((0, 0));
        if current_time - *last_seen_time >= TIME_WINDOW {
            if *count == 0 {
                println!("{}", line);
            } else {
                println!(
                    "{} (seen {} times in the last {} seconds)",
                    line, *count, TIME_WINDOW
                );
            }
            *last_seen_time = current_time;
            *count = 0;
        } else {
            *count += 1;
        }
    }
}
