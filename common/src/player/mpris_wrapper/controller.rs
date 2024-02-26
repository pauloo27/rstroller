use super::{super::get_player_by_bus_name, PlayerAction};
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
                    .expect("Failed to send error");
                return;
            }
            Err(e) => {
                ready_tx.send(Err(e)).expect("Failed to send error");
                return;
            }
        };

        ready_tx.send(Ok(())).expect("Failed to send bus name");

        while let Ok(action) = receiver.recv() {
            match action {
                PlayerAction::PlayPause => player.play_pause().expect("Failed to play/pause"),
                PlayerAction::Next => player.next().expect("Failed to play next"),
                PlayerAction::Previous => player.previous().expect("Failed to play previous"),
                PlayerAction::Raise => player.raise().expect("Failed to raise"),
                PlayerAction::Shuffle(v) => player.set_shuffle(v).expect("Failed to set shuffle"),
                PlayerAction::Volume(volume) => {
                    player.set_volume(volume).expect("Failed to set volume")
                }
            }
        }
    });

    ready_rx.recv().unwrap()
}
