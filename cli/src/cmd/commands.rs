use super::CommandExecContext;
use common;
use mpris::{DBusError, PlayerFinder};
use std::process;

pub fn help_cmd(ctx: CommandExecContext) {
    ctx.app.help();
}

pub fn play_cmd(ctx: CommandExecContext) {
    exec_player_action(ctx, "play", |player| player.play());
}

pub fn pause_cmd(ctx: CommandExecContext) {
    exec_player_action(ctx, "pause", |player| player.pause());
}

pub fn raise_cmd(ctx: CommandExecContext) {
    exec_player_action(ctx, "raise", |player| player.raise());
}

pub fn play_pause_cmd(ctx: CommandExecContext) {
    exec_player_action(ctx, "play/pause", |player| player.play_pause());
}

pub fn stop_cmd(ctx: CommandExecContext) {
    exec_player_action(ctx, "stop", |player| player.stop());
}

pub fn next_cmd(ctx: CommandExecContext) {
    exec_player_action(ctx, "next", |player| player.next());
}

pub fn previous_cmd(ctx: CommandExecContext) {
    exec_player_action(ctx, "previous", |player| player.previous());
}

pub fn set_preferred_player_cmd(ctx: CommandExecContext) {
    let player_name = ctx.args.args.get(2);

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
    let value = ctx.args.args.get(2);

    exec_player_action_silent(&ctx, "volume", |player| -> Result<(), DBusError> {
        match value {
            Some(value) => {
                let suffix = value.chars().last();
                if let None = suffix {
                    eprintln!("Invalid volume");
                    process::exit(0);
                }
                let suffix = suffix.unwrap();

                let offset = match suffix {
                    '+' => value
                        .trim_end_matches('+')
                        .parse::<f64>()
                        .expect("Invalid volume"),
                    '-' => {
                        value
                            .trim_end_matches('-')
                            .parse::<f64>()
                            .expect("Invalid volume")
                            * -1.0
                    }
                    _ => {
                        return player.set_volume(value.parse::<f64>().expect("Invalid volume"));
                    }
                };

                let current_volume = player.get_volume()?;
                player.set_volume(current_volume + offset)?;
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

pub fn exec_player_action<F>(ctx: CommandExecContext, action_name: &str, action: F)
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
