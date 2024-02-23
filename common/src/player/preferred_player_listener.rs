use std::{fs, path::Path, process};

use anyhow::{Context, Result as AnyResult};
use notify::{event::AccessKind, event::AccessMode, EventKind, Watcher};
use tokio::sync::mpsc::Receiver;

pub struct PreferredPlayerListener {
    watcher: Option<notify::RecommendedWatcher>,
}

impl PreferredPlayerListener {
    pub fn new() -> Self {
        PreferredPlayerListener { watcher: None }
    }

    pub fn start(&mut self) -> AnyResult<Receiver<AnyResult<String>>> {
        let path = Path::new(super::PREFERRED_PLAYER_FILE_PATH);
        if !path.exists() {
            fs::write(path, "").context("couldn't create the preferred player file")?;
        }

        let (tx, rx) = tokio::sync::mpsc::channel::<AnyResult<String>>(1);

        self.watcher = Some(notify::recommended_watcher(
            move |res: Result<notify::Event, notify::Error>| match res {
                Ok(event) => {
                    if event.kind == EventKind::Access(AccessKind::Close(AccessMode::Write)) {
                        let player_name = fs::read_to_string(super::PREFERRED_PLAYER_FILE_PATH);

                        tx.blocking_send(
                            player_name.context("Failed to read preferred player file"),
                        )
                        .expect("Failed to send message in mpsc");
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
