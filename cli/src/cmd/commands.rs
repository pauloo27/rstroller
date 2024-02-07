use super::CommandExecContext;
use common;
use mpris::PlayerFinder;

pub fn help_cmd(ctx: CommandExecContext) {
    ctx.app.help();
}

pub fn play_cmd(_: CommandExecContext) {
    common::get_player()
        .expect("Failed to get player")
        .play()
        .expect("Failed to play");
}

pub fn pause_cmd(_: CommandExecContext) {
    common::get_player()
        .expect("Failed to get player")
        .pause()
        .expect("Failed to play");
}

pub fn play_pause_cmd(_: CommandExecContext) {
    common::get_player()
        .expect("Failed to get player")
        .play_pause()
        .expect("Failed to play");
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
