use anyhow::{Context, Result as AnyResult};
use itertools::Itertools;
use mpris::{Player, PlayerFinder};
use std::{fs, io, path::Path};

mod watcher;
pub use watcher::*;

const PREFERRED_PLAYER_FILE_PATH: &str = "/dev/shm/rstroller-player";

pub fn get_preferred_player_name() -> AnyResult<Option<String>> {
    match fs::read_to_string(Path::new(PREFERRED_PLAYER_FILE_PATH)) {
        Ok(content) => Ok(Some(content.trim().to_string())),
        Err(err) if err.kind() == io::ErrorKind::NotFound => Ok(None),
        Err(err) => Err(err).context("couldn't read preferred player"),
    }
}

pub fn set_preferred_player_name(name: &str) -> AnyResult<()> {
    fs::write(Path::new(PREFERRED_PLAYER_FILE_PATH), name)
        .context("couldn't write the preferred player file")
}

pub fn get_player_by_bus_name(name: &str) -> AnyResult<Option<Player>> {
    let finder = PlayerFinder::new()?;

    Ok(finder
        .iter_players()?
        .process_results(|mut i| i.find(|p| p.bus_name() == name))?)
}

pub fn get_first_player() -> AnyResult<Option<Player>> {
    let finder = PlayerFinder::new()?;

    // QUESTION: which bear is best? well, there's basically two schools of thought
    Ok(finder.find_all()?.into_iter().next())
}

pub fn get_preferred_player() -> AnyResult<Option<Player>> {
    let name =
        get_preferred_player_name().context("couldn't get the name of the preferred player")?;

    match name {
        Some(name) => get_player_by_bus_name(&name),
        None => get_first_player(),
    }
}

pub fn get_preferred_player_or_first() -> AnyResult<Option<Player>> {
    match get_preferred_player() {
        Ok(None) => get_first_player(),
        x => x,
    }
}
