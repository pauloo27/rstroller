use super::CommandName;
use crate::core::CommandExecContext;
use mpris::{DBusError, Event};
use serde_json::json;
use std::process;

pub fn waybar_cmd(ctx: CommandExecContext<CommandName>) {
    if let Some(_) = ctx.args.flags.get("player") {
        eprintln!("Waybar mode does not support the --player flag");
        process::exit(1);
    };

    let mut had_prev_player = true;

    loop {
        let player = common::player::get_preferred_player_or_first().expect("Failed to get player");
        match player {
            Some(ref player) => handle_player(player),
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

fn handle_player(player: &mpris::Player) {
    show(player).expect("Failed to show player");

    for event in player.events().expect("Failed to get events") {
        if event.is_err() {
            break;
        }
        let event = event.unwrap();
        match event {
            Event::TrackChanged(_) | Event::Playing | Event::Paused | Event::Stopped => {
                show(player).expect("Failed to show player")
            }
            Event::PlayerShutDown => {
                break;
            }

            _ => {}
        }
    }
}

fn show(player: &mpris::Player) -> Result<(), DBusError> {
    let metadata = player.get_metadata()?;
    let title = metadata.title().expect("title not found");
    let artist = metadata.artists();
    let album = metadata.album_name();

    let icon = match player.get_playback_status()? {
        mpris::PlaybackStatus::Playing => "",
        mpris::PlaybackStatus::Paused => "",
        mpris::PlaybackStatus::Stopped => "",
    };

    let (line, tooltip) = match artist {
        Some(artist) => (
            format!(
                "{} {} by {}",
                icon,
                common::utils::truncate_string(title, 40),
                common::utils::truncate_string(&artist.join(", "), 20),
            ),
            format!(
                "{} by {}{}",
                title,
                &artist.join(", "),
                match album {
                    None => "".to_string(),
                    Some(album) => format!("from the album {}", album),
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
