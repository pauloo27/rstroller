mod waybar;

use super::utils;
use super::CommandExecContext;
use common;
use mpris::{DBusError, PlayerFinder};
use std::process;
use std::time::Duration;

pub use waybar::waybar_cmd;

pub fn help_cmd(ctx: CommandExecContext) {
    ctx.app.help();
}

pub fn play_cmd(ctx: CommandExecContext) {
    exec_player_action(&ctx, "play", |player| player.play());
}

pub fn pause_cmd(ctx: CommandExecContext) {
    exec_player_action(&ctx, "pause", |player| player.pause());
}

pub fn raise_cmd(ctx: CommandExecContext) {
    exec_player_action(&ctx, "raise", |player| player.raise());
}

pub fn play_pause_cmd(ctx: CommandExecContext) {
    exec_player_action(&ctx, "play/pause", |player| player.play_pause());
}

pub fn stop_cmd(ctx: CommandExecContext) {
    exec_player_action(&ctx, "stop", |player| player.stop());
}

pub fn next_cmd(ctx: CommandExecContext) {
    exec_player_action(&ctx, "next", |player| player.next());
}

pub fn previous_cmd(ctx: CommandExecContext) {
    exec_player_action(&ctx, "previous", |player| player.previous());
}

pub fn metadata_cmd(ctx: CommandExecContext) {
    exec_player_action_silent(&ctx, "metadata", |player| {
        let metadata_key = ctx.args.get(2);

        let metadata = player.get_metadata()?;

        match metadata_key {
            Some(metadata_key) => match metadata.get(metadata_key) {
                Some(value) => println!("{}", utils::format_metadata_value(value)),
                None => eprintln!("Metadata key not found"),
            },
            None => {
                for (key, value) in metadata {
                    println!("{}: {}", key, utils::format_metadata_value(&value));
                }
            }
        }

        Ok(())
    });
}

pub fn position_cmd(ctx: CommandExecContext) {
    let value = ctx.args.get(2);

    exec_player_action_silent(&ctx, "position", |player| -> Result<(), DBusError> {
        let metadata = player.get_metadata()?;

        let track_id = metadata.track_id().expect("No track id");

        match value {
            Some(value) => {
                let offset = match value.chars().last().expect("Invalid position") {
                    '+' => utils::parse_offset(value).expect("Invalid position"),
                    '-' => utils::parse_offset(value).expect("Invalid position") * -1.0,
                    _ => {
                        let value = value.parse::<u64>().expect("Invalid position");
                        return player.set_position(track_id, &Duration::from_millis(value));
                    }
                };

                let duration = player
                    .get_position()?
                    .checked_add(Duration::from_millis(offset as u64))
                    .expect("Invalid position");

                player.set_position(track_id, &duration)?
            }
            None => println!("{}", player.get_position()?.as_millis()),
        }
        Ok(())
    });
}

pub fn show_cmd(ctx: CommandExecContext) {
    exec_player_action_silent(&ctx, "show", |player| {
        println!("{} ({})", player.identity(), player.bus_name());
        println!("Playback status: {:?}", player.get_playback_status()?);
        println!("Volume: {}", player.get_volume()?);
        println!("Position: {}", player.get_position()?.as_millis());
        let metadata = player.get_metadata()?;

        println!("Metadata:");
        for (key, value) in metadata {
            println!("  {}: {}", key, utils::format_metadata_value(&value));
        }
        Ok(())
    });
}

pub fn loop_cmd(ctx: CommandExecContext) {
    exec_player_action_silent(&ctx, "loop", |player| {
        let value = ctx.args.get(2);

        match value {
            Some(v) => match v.to_lowercase().as_str() {
                "none" => player.set_loop_status(mpris::LoopStatus::None),
                "track" => player.set_loop_status(mpris::LoopStatus::Track),
                "playlist" => player.set_loop_status(mpris::LoopStatus::Playlist),
                _ => panic!("Invalid loop status"),
            },
            None => {
                println!("{:?}", player.get_loop_status()?);
                Ok(())
            }
        }
    });
}

pub fn shuffle_cmd(ctx: CommandExecContext) {
    exec_player_action_silent(&ctx, "shuffle", |player| {
        let value = ctx.args.get(2);

        match value {
            Some(v) => match v.to_lowercase().as_str() {
                "true" => player.set_shuffle(true),
                "false" => player.set_shuffle(false),
                _ => panic!("Invalid shuffle status"),
            },
            None => {
                println!("{:?}", player.get_shuffle()?);
                Ok(())
            }
        }
    });
}

pub fn set_preferred_player_cmd(ctx: CommandExecContext) {
    let player_name = ctx.args.get(2);

    match player_name {
        None => {
            eprintln!("Player name not provided");
            process::exit(1);
        }
        Some(player_name) => {
            common::set_preferred_player_name(&player_name)
                .expect("Failed to set preferred player name");
            println!("Preferred player set to {}", player_name);
        }
    }
}

pub fn volume_cmd(ctx: CommandExecContext) {
    let value = ctx.args.get(2);

    exec_player_action_silent(&ctx, "volume", |player| -> Result<(), DBusError> {
        match value {
            Some(value) => {
                let offset = match value.chars().last().expect("Invalid volume") {
                    '+' => utils::parse_offset(value).expect("Invalid volume"),
                    '-' => utils::parse_offset(value).expect("Invalid volume") * -1.0,
                    _ => {
                        return player.set_volume(value.parse::<f64>().expect("Invalid volume"));
                    }
                };

                player.set_volume(player.get_volume()? + offset)?;
            }
            None => println!("{}", player.get_volume()?),
        }
        Ok(())
    });
}

pub fn status_cmd(ctx: CommandExecContext) {
    exec_player_action_silent(&ctx, "status", |player| {
        let status = player.get_playback_status()?;
        println!("{:?}", status);
        Ok(())
    });
}

pub fn list_players_cmd(_: CommandExecContext) {
    let preferred_player_name = common::get_preferred_player_name()
        .expect("Failed to get preferred player name")
        .unwrap_or("".into());

    let players = PlayerFinder::new()
        .expect("Failed create PlayerFinder")
        .find_all()
        .expect("Failed to list players");

    if players.len() == 0 {
        println!("No players found");
        return;
    }

    for player in players {
        if player.bus_name() == preferred_player_name {
            println!("{}: {} (preferred)", player.identity(), player.bus_name());
        } else {
            println!("{}: {}", player.identity(), player.bus_name());
        }
    }
}

pub fn exec_player_action<F>(ctx: &CommandExecContext, action_name: &str, action: F)
where
    F: FnOnce(&mpris::Player) -> Result<(), DBusError>,
{
    exec_player_action_silent(&ctx, action_name, |player| {
        action(player)?;
        println!(
            "Action {action_name} called on player {} ({})",
            player.identity(),
            player.bus_name(),
        );
        Ok(())
    });
}

pub fn exec_player_action_silent<F>(ctx: &CommandExecContext, action_name: &str, action: F)
where
    F: FnOnce(&mpris::Player) -> Result<(), DBusError>,
{
    let player = match ctx.args.flags.get("player") {
        None => common::get_preferred_player_or_first().expect("Failed to get preferred player"),
        Some(player_name) => {
            common::get_player_by_bus_name(player_name).expect("Failed to get player")
        }
    };

    match player {
        Some(player) => {
            action(&player).expect(format!("Failed to call action {action_name}").as_str());
        }
        None => {
            eprintln!("No player found");
            process::exit(1);
        }
    }
}
