use super::PlayerState;
use std::{process, thread};

pub fn spawn_mpris_listener(sender: async_channel::Sender<PlayerState>) {
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

        let mut last_player_state = PlayerState::new(&player);

        // send initial player state
        sender
            .send_blocking(last_player_state.clone())
            .expect("Failed to send metadata");

        for event in events {
            last_player_state =
                last_player_state.handle_event(event.expect("Failed to read mpris event"));
            sender
                .send_blocking(last_player_state.clone())
                .expect("Failed to send metadata");
        }
    });
}
