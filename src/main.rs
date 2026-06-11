use std::time::{Duration, Instant};
use warptrixy::{Config, CustomAppDto, TextValue, WarptrixyClient};

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

    let code = client
        .send_notification(warptrixy::CustomAppDto {
            text: Option::from(TextValue::String("TEST".to_string())),
            text_case: None,
            top_text: None,
            text_offset: None,
            center: None,
            color: None,
            gradient: None,
            blink_text: None,
            fade_text: None,
            background: None,
            rainbow: None,
            icon: None,
            push_icon: None,
            repeat: None,
            duration: None,
            hold: None,
            sound: None,
            rtttl: None,
            loop_sound: None,
            bar: None,
            line: None,
            autoscale: None,
            bar_bc: None,
            progress: None,
            progress_c: None,
            progress_bc: None,
            pos: None,
            draw: None,
            lifetime: None,
            lifetime_mode: None,
            stack: None,
            wakeup: None,
            no_scroll: None,
            clients: None,
            scroll_speed: None,
            effect: None,
            effect_settings: None,
            save: None,
            overlay: None,
        })
        .await
        .unwrap();
    println!("{}", code);
    println!("{:?}", client.set_power(true).await.unwrap());

  /*
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
   */
}
