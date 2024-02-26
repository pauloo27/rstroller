use super::{
    controller::spawn_mpris_controller, listener::spawn_mpris_listener, PlayerAction, PlayerState,
};
use anyhow::Result as AnyResult;
use std::sync::mpsc;
use tokio::sync::mpsc as tokio_mpsc;

pub struct MprisWrapper {
    player_name: String,
}

impl MprisWrapper {
    pub fn new(player_name: String) -> Self {
        MprisWrapper { player_name }
    }

    pub fn start_listener(&self, sender: tokio_mpsc::Sender<PlayerState>) -> AnyResult<()> {
        spawn_mpris_listener(self.player_name.clone(), sender)
    }

    pub fn start_controller(&self, receiver: mpsc::Receiver<PlayerAction>) -> AnyResult<()> {
        spawn_mpris_controller(self.player_name.clone(), receiver)
    }
}
