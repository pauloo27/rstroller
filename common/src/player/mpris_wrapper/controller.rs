use super::{super::get_player_by_bus_name, PlayerAction};
use crate::err::*;
use anyhow::Result as AnyResult;
use std::sync::mpsc;
use std::thread;

pub fn spawn_mpris_controller(
    player_name: String,
    receiver: mpsc::Receiver<PlayerAction>,
) -> AnyResult<()> {
    let (ready_tx, ready_rx) = mpsc::channel();

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

        ready_tx.send(Ok(())).or_exit("Failed to send ready signal");

        while let Ok(action) = receiver.recv() {
            match action {
                PlayerAction::PlayPause => player.play_pause().log_err("Failed to play/pause"),
                PlayerAction::Next => player.next().log_err("Failed to play next"),
                PlayerAction::Previous => player.previous().log_err("Failed to play previous"),
                PlayerAction::Seek(v) => {
                    let track_id = player
                        .get_metadata()
                        .or_exit("Failed to get metadata")
                        .track_id();
                    if let Some(track_id) = track_id {
                        player.set_position(track_id, &v).log_err("Failed to seek")
                    }
                }
                PlayerAction::Raise => player.raise().log_err("Failed to raise"),
                PlayerAction::Shuffle(v) => player.set_shuffle(v).log_err("Failed to set shuffle"),
                PlayerAction::Volume(volume) => {
                    player.set_volume(volume).log_err("Failed to set volume")
                }
            }
        }
    });

    ready_rx.recv().or_exit("Failed to receive ready signal")
}
