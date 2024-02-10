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

pub fn list_players_cmd(_: CommandExecContext) {
    let players = PlayerFinder::new()
        .expect("Failed create PlayerFinder")
        .find_all()
        .expect("Failed to list players");

    if players.len() == 0 {
        println!("No players found");
        return;
    }

    for player in players {
        println!("{}: {}", player.identity(), player.bus_name());
    }
}

pub fn exec_player_action<F>(ctx: CommandExecContext, action_name: &str, action: F)
where
    F: FnOnce(&mpris::Player) -> Result<(), DBusError>,
{
    let player = match ctx.args.flags.get("player") {
        None => common::get_preferred_player().expect("Failed to get preferred player"),
        Some(player_name) => {
            common::get_player_by_bus_name(player_name).expect("Failed to get player")
        }
    };

    match player {
        Some(player) => {
            action(&player).expect(format!("Failed to call action {action_name}").as_str());
            println!(
                "Action {action_name} called on player {} ({})",
                player.identity(),
                player.bus_name(),
            );
        }
        None => {
            eprintln!("No player found");
            process::exit(1);
        }
    }
}
