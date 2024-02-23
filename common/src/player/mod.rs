use anyhow::Context;
use anyhow::Result as AnyResult;
use itertools::Itertools;
use mpris::{Player, PlayerFinder};
use std::path::Path;
use std::{fs, io};

mod mpris_listener;
mod player_state;
mod preferred_player_listener;

pub use mpris_listener::*;
pub use player_state::*;
pub use preferred_player_listener::*;

const PREFERRED_PLAYER_FILE_PATH: &'static str = "/dev/shm/rstroller-player";

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

    // Finds the first player with the requested bus name
    Ok(finder
        .iter_players()?
        // Processes this iterator of results as if it was an iterator of the
        // Ok values. If an error is found, it's returned.
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
