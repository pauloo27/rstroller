use std::sync::{Arc, RwLock};

use mpris::{FindingError, PlayerFinder};

pub struct ServerState {
    pub preferred_player: Arc<RwLock<String>>,
}

impl ServerState {
    pub fn load_initial() -> Result<Self, FindingError> {
        let finder = PlayerFinder::new()?;

        let active_player = match finder.find_active() {
            Ok(p) => p.bus_name().to_string(),
            Err(FindingError::NoPlayerFound) => "".to_string(),
            Err(e) => return Err(e),
        };

        Ok(ServerState {
            preferred_player: Arc::new(RwLock::new(active_player)),
        })
    }
}
