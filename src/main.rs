extern crate bcrypt;

use bcrypt::{DEFAULT_COST, hash, verify};
use std::time::Instant;

fn main() {
    for i in 0..15 {
        let now = Instant::now();

        {
            let hashed = match hash("B4c0/\\/", 15) {
                Ok(h) => println!("hash: {}", h),
                Err(_) => panic!()
            };
        }

        let elapsed = now.elapsed();
        let sec = (elapsed.as_secs() as f64) + (elapsed.subsec_nanos() as f64 / 1000_000_000.0);

        println!("crypt: {}", sec);
    }
    
}


