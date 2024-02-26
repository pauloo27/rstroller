use super::CommandName;
use crate::core_definition::CommandExecContext;
use anyhow::Result as AnyResult;
use common::player::{MprisWrapper, PlayerState};
use mpris::DBusError;
use serde_json::json;
use std::process;
use tokio::sync::mpsc;
use tokio::sync::mpsc::Receiver;

pub fn waybar_cmd(ctx: CommandExecContext<CommandName>) {
    if let Some(_) = ctx.args.flags.get("player") {
        eprintln!("Waybar mode does not support the --player flag");
        process::exit(1);
    };

    tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap()
        .block_on(start_waybar_loop());
}

pub async fn start_waybar_loop() {
    let mut listener = common::player::PreferredPlayerListener::new();
    let mut player_rx = listener.start().expect("Failed to start listener");

    let mut had_prev_player = true;

    loop {
        let player = common::player::get_preferred_player_or_first().expect("Failed to get player");

        match player {
            Some(ref player) => {
                let player_name = player.bus_name();

                let wrapper = MprisWrapper::new(player_name.to_string());
                let (event_tx, event_rx) = mpsc::channel(1);

                common::player::set_preferred_player_name(player_name)
                    .expect("Failed to set player");

                wrapper
                    .start_listener(event_tx)
                    .expect("Failed to start listener");

                player_rx = handle_player(player_name, event_rx, player_rx).await;
            }
            None => {
                if had_prev_player {
                    println!(
                        "{}",
                        json!({"text": "Silence", "tooltip": "Nothing playing"}).to_string()
                    );
                }
                std::thread::sleep(std::time::Duration::from_secs(1));
            }
        }
        had_prev_player = player.is_some();
    }
}

async fn handle_player(
    player_name: &str,
    mut event_rx: Receiver<PlayerState>,
    mut player_rx: Receiver<AnyResult<String>>,
) -> Receiver<AnyResult<String>> {
    loop {
        tokio::select! {
            state = event_rx.recv() => {
                if let Some(state) = state {
                    show(state).expect("Failed to show player state");
                } else {
                    break;
                }
            },
            new_player_name = player_rx.recv() => {
                match new_player_name {
                    Some(Ok(new_player_name)) => {
                        if new_player_name != player_name {
                            break;
                        }
                    },
                    _ => break,
                }
            }
        }
    }
    player_rx
}

fn show(state: PlayerState) -> Result<(), DBusError> {
    let metadata = state.metadata;

    let title = metadata.title().unwrap_or("Unknown title");
    let artists = parse_artists(metadata.artists());
    let album = metadata.album_name();

    let icon = match state.playback_status {
        mpris::PlaybackStatus::Playing => "",
        mpris::PlaybackStatus::Paused => "",
        mpris::PlaybackStatus::Stopped => "",
    };

    let (line, tooltip) = match artists {
        Some(artists) => (
            format!(
                "{} {} by {}",
                icon,
                common::utils::truncate_string(title, 40),
                common::utils::truncate_string(&artists, 20),
            ),
            format!(
                "{} by {}{}",
                title,
                artists,
                match album {
                    Some(album) if !album.is_empty() => format!(" from the album {}", album),
                    _ => "".to_string(),
                }
            ),
        ),
        None => (
            format!("{} {}", icon, common::utils::truncate_string(title, 40)),
            format!("{}", title),
        ),
    };

    let output = json!({
        "text": line,
        "tooltip": tooltip,
    });

    println!("{}", output.to_string());

    Ok(())
}

fn parse_artists(artists: Option<Vec<&str>>) -> Option<String> {
    match artists {
        Some(artists) => {
            if artists.is_empty() {
                return None;
            }
            let joined_artists = artists.join(", ");

            if joined_artists == "" {
                return None;
            }

            Some(joined_artists)
        }
        None => None,
    }
}
