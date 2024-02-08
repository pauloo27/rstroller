use mpris;
use std::fs;
use std::path::Path;

const PREFERRED_PLAYER_FILE_PATH: &'static str = "/dev/shm/rstroller-player";

pub fn get_preferred_player() -> Result<Option<String>, String> {
    match fs::read_to_string(Path::new(PREFERRED_PLAYER_FILE_PATH)) {
        Ok(content) => Ok(Some(content.trim().to_string())),
        Err(err) => {
            if err.kind() == std::io::ErrorKind::NotFound {
                Ok(None)
            } else {
                Err(err.to_string())
            }
        }
    }
}

pub fn get_player() -> Result<Option<mpris::Player>, String> {
    let finder = mpris::PlayerFinder::new().map_err(|e| e.to_string())?;
    let preferred_player = get_preferred_player()?;

    if let Some(preferred_player) = preferred_player {
        let player = finder.find_by_name(preferred_player.as_str());
        return player.map_err(|e| e.to_string()).map(|p| Some(p));
    }

    Ok(finder
        .find_all()
        .map_err(|e| e.to_string())?
        .into_iter()
        .next())
}
