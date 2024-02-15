use std::path::Path;
use std::{fs, io};

use anyhow::Context;
use anyhow::Result as AnyResult;
use itertools::Itertools;
use mpris::{Player, PlayerFinder};

const PREFERRED_PLAYER_FILE_PATH: &'static str = "/dev/shm/rstroller-player";

/// Gets a name from the preferred player file.
///
/// # Output
///
/// If no errors occur, it returns the trimmed (and cloned) contents of the
/// file or [None] if the file doesn't exist.
///
/// # Errors
///
/// Uses [anyhow] to propagate any errors from [fs::read_to_string] except if
/// its [kind](io::Error::kind) is [NotFound](io::ErrorKind::NotFound),
/// returning [Ok(None)] in this case.
///
pub fn get_preferred_player_name() -> AnyResult<Option<String>> {
    match fs::read_to_string(Path::new(PREFERRED_PLAYER_FILE_PATH)) {
        Ok(content) => Ok(Some(content.trim().to_string())),
        Err(err) if err.kind() == io::ErrorKind::NotFound => Ok(None),
        Err(err) => Err(err).context("couldn't read preferred player"),
    }
}

/// Writes the name to the preferred player file.
///
/// # Errors
///
/// Propagates [fs::write] with [anyhow].
///
pub fn set_preferred_player_name(name: &str) -> AnyResult<()> {
    fs::write(Path::new(PREFERRED_PLAYER_FILE_PATH), name)
        .context("couldn't write the preferred player file")
}

/// Finds a player inside [PlayerFinder] with a matching bus name
///
/// # Output
///
/// When no errors occur, it returns the first player in the list matching the
/// name or [None] if it isn't found.
///
/// # Errors
///
/// Propagates [PlayerFinder::new] and [PlayerFinder::iter_players] with
/// [anyhow].
///
pub fn get_player_by_bus_name(name: &str) -> AnyResult<Option<Player>> {
    let finder = PlayerFinder::new()?;

    // Finds the first player with the requested bus name
    Ok(finder
        .iter_players()?
        // Processes this iterator of results as if it was an iterator of the
        // Ok values. If an error is found, it's returned.
        .process_results(|mut i| i.find(|p| p.bus_name() == name))?)
}

/// Gets the first player it can find
///
/// # Output
///
/// Some random player
///
/// # Errors
///
/// Propagates any DBus error.
pub fn get_first_player() -> AnyResult<Option<Player>> {
    let finder = PlayerFinder::new()?;

    // QUESTION: which bear is best? well, there's basically two schools of thought
    Ok(finder.find_all()?.into_iter().next())
}

/// Gets the preferred player from the file.
///
/// # Output
///
/// If the preferred player file exists, it tries to search for the player and
/// returns [None] if the player isn't found. Otherwise (the file doesn't
/// exist), it returns the first player it can find using [get_first_player].
///
/// # Errors
///
/// If there was a problem opening the file (except it not existing) or if any
/// problem with DBus happened the error is propagated with [anyhow].
///
pub fn get_preferred_player() -> AnyResult<Option<Player>> {
    let name =
        get_preferred_player_name().context("couldn't get the name of the preferred player")?;

    match name {
        Some(name) => get_player_by_bus_name(&name),
        None => get_first_player(),
    }
}

/// Similar to [get_preferred_player], but returns the first player it can find
/// even if the preferred player file exists.
///
/// # Output
///
/// Tries to search for the preferred player and returns it. If the preferred
/// player file doesn't exist or if the preferred player can't be found, it
/// returns the first player it can find. [None] is only returned if there are
/// no players available.
///
/// # Errors
///
/// Propagates any error from [get_preferred_player] or from [get_first_player]
///
pub fn get_preferred_player_or_first() -> AnyResult<Option<Player>> {
    match get_preferred_player() {
        Ok(None) => get_first_player(),
        x => x,
    }
}
