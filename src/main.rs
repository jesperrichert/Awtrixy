use std::time::{Duration, Instant};

fn main() {
    let host = std::env::var("AWTRIX_WEB_URL").unwrap();
    let user = std::env::var("AWTRIX_USER").unwrap();
    let password = std::env::var("AWTRIX_PASSWORD").unwrap();
    println!("Started Awtrixy for connection {}", host);

    let client = "";

    let now = Instant::now();
    let run_every = Duration::from_secs(10);
    let mut next_run = now + run_every;
    loop {
        println!("Run Every? {}", (now - next_run).as_secs());
        if (now - next_run).as_secs() > 1 {
            next_run = Instant::now() + run_every;
            println!("Running Scheduler for Display Update")
        }
    }
}
