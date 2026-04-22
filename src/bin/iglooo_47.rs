use sha2::{Digest, Sha256};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::thread;

fn main() {
    let found = Arc::new(AtomicBool::new(false));
    let num_threads = num_cpus::get();
    let mut handles = Vec::new();
    let block = "hello_world_2";

    for i in 0..num_threads {
        let found = Arc::clone(&found);
        let start = (i as u64) * 1_000_000;
        let end = start + 1_000_000;

        let handle = thread::spawn(move || {
            println!("thread {} started | range {}..{}", i, start, end);

            for nonce in start..end {
                if found.load(Ordering::Relaxed) {
                    break;
                } else {
                    let input = format!("{}{}", block, nonce);
                    let hash = Sha256::digest(&input);

                    if hash.starts_with(&[0, 0]) {
                        println!("thread {} found nonce: {}", i, nonce);
                        println!("hash: {:x}", hash);

                        found.store(true, Ordering::Relaxed);
                        break;
                    }
                }
            }
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }
}
