use super::PlayerState;
use std::{process, thread};
use tokio::sync::mpsc;

pub fn spawn_mpris_listener(sender: mpsc::Sender<PlayerState>) {
    thread::spawn(move || {
        let player = match super::get_preferred_player_or_first() {
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
            .blocking_send(last_player_state.clone())
            .expect("Failed to send metadata");

        for event in events {
            last_player_state =
                last_player_state.handle_event(event.expect("Failed to read mpris event"));
            sender
                .blocking_send(last_player_state.clone())
                .expect("Failed to send metadata");
        }
    });
}
