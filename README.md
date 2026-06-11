# Warptrixy

> Simple and easy to use API Wrapper for the Open Source firmware "AWTRIX 3".

## Installation
```shell
cargo add warptrixy
```

## Usage
```rust
        let client = Warptrixy::new(Config {
            url: "http://192.168.178.10".to_string(),
            username: "jesforge".to_string(),
            password: "ZpSIuaCLY3fAE2PF1wMStOb1d".to_string(),
        });
        client.send_notification(
            CustomAppDto {
                text: Option::from(TextValue::String("test".to_string())),
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
            }
        );
    }
```