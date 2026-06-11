mod disbot_app;
mod twitch_app;

use crate::disbot_app::disbot_app;
use std::time::{Duration, Instant};
use warptrixy::{Config, CustomAppDto, TextValue, WarptrixyClient};
use crate::twitch_app::twitch_app;

#[tokio::main]
async fn main() {
    let host = std::env::var("AWTRIX_WEB_URL").unwrap();
    let user = std::env::var("AWTRIX_USER").unwrap();
    let password = std::env::var("AWTRIX_PASSWORD").unwrap();
    println!("Started Awtrixy for connection {}", host);

    let client = warptrixy::WarptrixyClient::new(warptrixy::Config {
        url: host.to_string(),
        username: user.to_string(),
        password: password.to_string(),
    });

    let time = Instant::now();
    let run_every = Duration::from_mins(5);
    loop {
        if Instant::now() - time >= run_every {
            println!("Time taken to run the following events...");

            disbot_app(&client).await;
            twitch_app(&client).await;
        }
    }
}
