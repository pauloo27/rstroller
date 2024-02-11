use std::fs;
use std::path::Path;

const PREFERRED_PLAYER_FILE_PATH: &'static str = "/dev/shm/rstroller-player";

pub fn get_preferred_player_name() -> Result<Option<String>, String> {
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

pub fn set_preferred_player_name(name: &str) -> Result<(), String> {
    fs::write(Path::new(PREFERRED_PLAYER_FILE_PATH), name).map_err(|e| e.to_string())
}

pub fn get_player_by_bus_name(name: &str) -> Result<Option<mpris::Player>, String> {
    let finder = mpris::PlayerFinder::new().map_err(|e| e.to_string())?;
    for player in finder.iter_players().map_err(|e| e.to_string())? {
        let player = player.map_err(|e| e.to_string())?;
        if player.bus_name() == name {
            return Ok(Some(player));
        }
    }

    Ok(None)
}

pub fn get_preferred_player() -> Result<Option<mpris::Player>, String> {
    let finder = mpris::PlayerFinder::new().map_err(|e| e.to_string())?;
    let preferred_player = get_preferred_player_name()?;

    if let Some(preferred_player) = preferred_player {
        for player in finder.iter_players().map_err(|e| e.to_string())? {
            let player = player.map_err(|e| e.to_string())?;
            if player.bus_name() == preferred_player {
                return Ok(Some(player));
            }
        }

        return Ok(None);
    }

    Ok(finder
        .find_all()
        .map_err(|e| e.to_string())?
        .into_iter()
        .next())
}

pub fn get_preferred_player_or_first() -> Result<Option<mpris::Player>, String> {
    let preferred_player = get_preferred_player()?;
    if let Some(preferred_player) = preferred_player {
        return Ok(Some(preferred_player));
    }

    let finder = mpris::PlayerFinder::new().map_err(|e| e.to_string())?;
    Ok(finder
        .find_all()
        .map_err(|e| e.to_string())?
        .into_iter()
        .next())
}
