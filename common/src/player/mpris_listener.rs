use super::PlayerState;
use anyhow::Result as AnyResult;
use std::{process, thread};
use tokio::sync::mpsc;

pub fn spawn_mpris_listener(sender: mpsc::Sender<PlayerState>) -> AnyResult<Option<String>> {
    let (ready_tx, ready_rx) = std::sync::mpsc::channel::<AnyResult<Option<String>>>();

    thread::spawn(move || {
        let player = match super::get_preferred_player_or_first() {
            Ok(Some(player)) => player,
            Ok(None) => {
                ready_tx.send(Ok(None)).expect("Failed to send None");
                return;
            }
            Err(e) => {
                ready_tx.send(Err(e)).expect("Failed to send error");
                return;
            }
        };

        ready_tx
            .send(Ok(Some(player.identity().to_string())))
            .expect("Failed to send identity");

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

    ready_rx.recv().unwrap()
}
