use super::super::get_player_by_bus_name;
use super::PlayerState;
use crate::err::*;
use anyhow::Result as AnyResult;
use std::{process, thread};
use tokio::sync::mpsc;

pub fn spawn_mpris_listener(
    player_name: String,
    sender: mpsc::Sender<PlayerState>,
) -> AnyResult<()> {
    let (ready_tx, ready_rx) = std::sync::mpsc::channel();

    thread::spawn(move || {
        let player = match get_player_by_bus_name(&player_name) {
            Ok(Some(player)) => player,
            Ok(None) => {
                ready_tx
                    .send(Err(anyhow::anyhow!("Player not found")))
                    .or_exit("Failed to send error");
                return;
            }
            Err(e) => {
                ready_tx.send(Err(e)).or_exit("Failed to send error");
                return;
            }
        };

        ready_tx.send(Ok(())).or_exit("Failed to send bus name");

        let events = player.events().unwrap_or_else(|e| {
            eprintln!("Error: {}", e);
            process::exit(1);
        });

        let mut player_state = PlayerState::new(&player);

        // send initial player state
        sender
            .blocking_send(player_state.clone())
            .or_exit("Failed to send initial state");

        for event in events {
            match event {
                Ok(event) => {
                    player_state = match player_state.handle_event(event) {
                        Some(state) => state,
                        None => break,
                    };

                    // the channed was closed...
                    // FIXME: close this thread earlier
                    if sender.blocking_send(player_state.clone()).is_err() {
                        break;
                    }
                }
                Err(_) => {
                    break;
                }
            }
        }
    });

    ready_rx.recv().unwrap()
}
