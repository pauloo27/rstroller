use crate::err::*;
use anyhow::{Context, Result as AnyResult};
use notify::{event::AccessKind, event::AccessMode, EventKind, Watcher};
use std::{fs, path::Path, process};
use tokio::sync::mpsc::Receiver;

use super::PREFERRED_PLAYER_FILE_PATH;

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

        Ok(rx)
    }
}

impl Default for PreferredPlayerListener {
    fn default() -> Self {
        Self::new()
    }
}
