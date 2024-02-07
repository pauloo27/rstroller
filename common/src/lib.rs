use mpris;
use std::fs;
use std::path::Path;

const PREFERRED_PLAYER_FILE_PATH: &'static str = "/dev/shm/rstroller-player";

pub fn get_preferred_player() -> Option<String> {
    match fs::read_to_string(Path::new(PREFERRED_PLAYER_FILE_PATH)) {
        Ok(content) => Some(content.trim().to_string()),
        Err(_) => None,
    }
}

pub fn get_player() -> Option<mpris::Player> {
    let finder = mpris::PlayerFinder::new().expect("Failed to create PlayerFinder");
    let preferred_player = get_preferred_player();

    if let Some(preferred_player) = preferred_player {
        let player = finder.find_by_name(preferred_player.as_str());
        if let Ok(player) = player {
            return Some(player);
        }
    }

    finder.find_all().ok()?.into_iter().next()
}
