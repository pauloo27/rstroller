use std::{fs, io, path::Path, process};

use crate::err::*;
use anyhow::{Context, Result as AnyResult};
use itertools::Itertools;
use mpris::{Player, PlayerFinder};
use notify::{event::AccessKind, event::AccessMode, EventKind, Watcher};
use tokio::sync::mpsc::Receiver;

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

pub struct PreferredPlayerListener {
    watcher: Option<notify::RecommendedWatcher>,
}

impl PreferredPlayerListener {
    pub fn new() -> Self {
        PreferredPlayerListener { watcher: None }
    }

    pub fn start(&mut self) -> AnyResult<Receiver<AnyResult<String>>> {
        let path = Path::new(PREFERRED_PLAYER_FILE_PATH);
        if !path.exists() {
            fs::write(path, "").context("couldn't create the preferred player file")?;
        }

        let (tx, rx) = tokio::sync::mpsc::channel::<AnyResult<String>>(1);

        self.watcher = Some(notify::recommended_watcher(
            move |res: Result<notify::Event, notify::Error>| match res {
                Ok(event) => {
                    if event.kind == EventKind::Access(AccessKind::Close(AccessMode::Write)) {
                        let player_name = fs::read_to_string(PREFERRED_PLAYER_FILE_PATH);

                        tx.blocking_send(
                            player_name.context("Failed to read preferred player file"),
                        )
                        .or_exit("Failed to send message in mpsc");
                    }
                }
                Err(_) => {
                    eprintln!("Failed to watch to preferred player file");
                    process::exit(1);
                }
            },
        )?);

        self.watcher
            .as_mut()
            .unwrap()
            .watch(path, notify::RecursiveMode::NonRecursive)?;

        return Ok(rx);
    }
}
