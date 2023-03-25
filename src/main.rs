use sha2::{Digest, Sha256};
use std::collections::HashMap;
use std::io;
use std::thread;
use std::time::{Duration, SystemTime};

// Define the time window for deduplication
const TIME_WINDOW: u64 = 5;

fn main() {
    let mut buffer = String::new();

    loop {
        match io::stdin().read_line(&mut buffer) {
            Ok(n) => {
                if n == 0 {
                    break;
                }
                let data = buffer.clone();
                thread::spawn(|| process_data(data));
                // Clear the buffer for the next iteration
                buffer.clear();
            }
            Err(e) => {
                eprintln!("Error reading from stdin: {}", e);
                break;
            }
        }
        // need to remove the need for this sleep as it slows down ingestion.
        thread::sleep(Duration::from_millis(1));
    }
}



fn process_data(data: String) {
    // Create an empty hashmap to store seen log signatures and their last seen timestamps
    let mut seen_signatures: HashMap<String, u64> = HashMap::new();

    let mut hasher = Sha256::new();

    let lines = data.split('\n');

    for line in lines {
        if line.is_empty() {
            continue;
        }
        let parts: Vec<&str> = line.split(" ").collect();
        let _timestamp = parts[0];
        let method = parts[1];
        let path = parts[2];
        let status = parts[3];
        let bytes_sent = parts[4];
        let referer = parts[5];
        let user_agent = parts[6..].join(" ").trim().to_owned();
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
        let last_seen_time = seen_signatures.get(&signature_hash_hex).unwrap_or(&0);
        if current_time - last_seen_time >= TIME_WINDOW {
            println!("{}", line);
            seen_signatures.insert(signature_hash_hex, current_time);
        } else {
            seen_signatures.insert(signature_hash_hex, current_time);
            let seen_count = seen_signatures
                .values()
                .filter(|&&ts| current_time - ts < TIME_WINDOW)
                .count();
            println!(
                "{} (seen {} times in the last {} seconds)",
                line, seen_count, TIME_WINDOW
            );
        }
    }
}
