use super::utils;
use super::utils::exec_player_action_silent;
use super::CommandName;
use crate::core::CommandExecContext;
use mpris::DBusError;
use serde_json::json;

pub fn waybar_cmd(ctx: CommandExecContext<CommandName>) {
    exec_player_action_silent(&ctx, "polybar", |player| {
        let events = player.events()?;

        show(&player)?;

        for _event in events {
            show(&player)?;
        }

        Ok(())
    });
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
                utils::truncate_string(title, 40),
                utils::truncate_string(&artist.join(", "), 20),
            ),
            format!(
                "{} by {}{}",
                title,
                &artist.join(", "),
                match album {
                    None => "".to_string(),
                    Some(album) => format!(" from the album {}", album),
                }
            ),
        ),
        None => (
            format!("{} {}", icon, utils::truncate_string(title, 40)),
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
