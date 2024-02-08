use super::CommandExecContext;
use common;
use mpris::{DBusError, PlayerFinder};

pub fn help_cmd(ctx: CommandExecContext) {
    ctx.app.help();
}

pub fn play_cmd(_: CommandExecContext) {
    exec_player_action(|player| player.play(), "play");
}

pub fn pause_cmd(_: CommandExecContext) {
    exec_player_action(|player| player.pause(), "pause");
}

pub fn play_pause_cmd(_: CommandExecContext) {
    exec_player_action(|player| player.play_pause(), "play/pause");
}

pub fn stop_cmd(_: CommandExecContext) {
    exec_player_action(|player| player.stop(), "stop");
}

pub fn next_cmd(_: CommandExecContext) {
    exec_player_action(|player| player.next(), "next");
}


pub fn previous_cmd(_: CommandExecContext) {
    exec_player_action(|player| player.previous(), "previous");
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

pub fn exec_player_action<F>(action: F, action_name: &str)
where
    F: FnOnce(mpris::Player) -> Result<(), DBusError>,
{
    let player = common::get_player().expect("Failed to get player");
    match player {
        Some(player) => {
            action(player).expect(format!("Failed to call action {action_name}").as_str())
        }
        None => eprintln!("No player found"),
    }
}
