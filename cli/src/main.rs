mod cmd;

use cmd::{App, Command, CommandName};
use mpris::PlayerFinder;
use std::rc::Rc;

fn main() {
    let mut app = new_app();
    if let None = app.run() {
        app.help();
    }
}

fn new_app() -> App<'static> {
    App::new(
        String::from("rstroller"),
        String::from("player controller for MPRIS"),
    )
    .add_command(Rc::new(Command::new(
        CommandName::ListPlayers,
        String::from("list available MPRIS players"),
        &list_players_cmd,
    )))
    .add_command(Rc::new(Command::new(
        CommandName::Play,
        String::from("send the play command to the preferred player"),
        &play_cmd,
    )))
    .add_command(Rc::new(Command::new(
        CommandName::Pause,
        String::from("send the pause command to the preferred player"),
        &pause_cmd,
    )))
    .add_command(Rc::new(Command::new(
        CommandName::PlayPause,
        String::from("send the play/pause command to the preferred player"),
        &play_pause_cmd,
    )))
}

fn list_players_cmd(_: Rc<Command>) {
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

fn play_cmd(_: Rc<Command>) {
    common::get_player()
        .expect("Failed to get player")
        .play()
        .expect("Failed to play");
}

fn pause_cmd(_: Rc<Command>) {
    common::get_player()
        .expect("Failed to get player")
        .pause()
        .expect("Failed to play");
}

fn play_pause_cmd(_: Rc<Command>) {
    common::get_player()
        .expect("Failed to get player")
        .play_pause()
        .expect("Failed to play");
}
