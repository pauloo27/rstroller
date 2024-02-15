use std::{process, thread};

pub fn spawn_mpris_listener(sender: async_channel::Sender<mpris::Metadata>) {
    thread::spawn(move || {
        let player = match common::get_preferred_player_or_first() {
            Ok(Some(player)) => player,
            Ok(None) => {
                eprintln!("Player not found");
                process::exit(1);
            }
            Err(e) => {
                eprintln!("Error: {}", e);
                process::exit(1);
            }
        };

        let events = player.events().unwrap_or_else(|e| {
            eprintln!("Error: {}", e);
            process::exit(1);
        });

        for _ in events {
            let metadata = player.get_metadata();
            sender
                .send_blocking(metadata.expect("Failed to get metadata"))
                .expect("Failed to send metadata");
        }
    });
}
