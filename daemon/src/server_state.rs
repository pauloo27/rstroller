use std::{
    sync::{Arc, RwLock},
    thread,
    time::Duration,
};

use mpris::{FindingError, PlayerFinder};

pub struct ServerState {
    pub preferred_player: Arc<RwLock<String>>,
}

impl ServerState {
    pub fn load_initial() -> Result<Self, FindingError> {
        let active_player = Self::find_active_player()?;

        Ok(ServerState {
            preferred_player: Arc::new(RwLock::new(active_player)),
        })
    }

    fn find_active_player() -> Result<String, FindingError> {
        let finder = PlayerFinder::new()?;

        match finder.find_active() {
            Ok(p) => Ok(p.bus_name().to_string()),
            Err(FindingError::NoPlayerFound) => Ok("".to_string()),
            Err(e) => Err(e),
        }
    }

    pub fn watch_for_changes(&self) {
        loop {
            let preferred_player = match self.preferred_player.read() {
                Ok(p) => p,
                Err(e) => {
                    eprintln!("Failed to acquire read lock: {e}");
                    continue;
                }
            };

            match preferred_player.as_str() {
                "" => {
                    drop(preferred_player);
                    match Self::find_active_player() {
                        Ok(player) if !player.is_empty() => match self.preferred_player.write() {
                            Ok(mut p) => {
                                // if the values was changed between the "drop read lock, get write
                                // lock", we just skip
                                if !p.is_empty() {
                                    continue;
                                }

                                *p = player.clone();
                                self.emit_property_changed();
                            }
                            Err(e) => eprintln!("Failed to acquire write lock: {e}"),
                        },
                        Ok(_) => {
                            thread::sleep(Duration::from_millis(200));
                        }
                        Err(e) => eprintln!("Failed to find active player: {e}"),
                    }
                }
                _ => {
                    // TODO: watch for events of that player, if closed, determine the new one
                    // again
                }
            }
        }
    }

    fn emit_property_changed(&self) {
        // TODO:
    }
}
