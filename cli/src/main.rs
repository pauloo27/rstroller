mod cmd;

use cmd::{commands, App, Command, CommandName};

fn main() {
    let mut app = new_app();
    if let None = app.run_cmd() {
        println!("Unknown command");
        app.help();
    }
}

fn new_app() -> App<'static> {
    App::new(
        String::from("rstroller"),
        String::from("player controller for MPRIS"),
    )
    .add_command(Command::new(
        CommandName::Help,
        String::from("show command usage"),
        &commands::help_cmd,
    ))
    .add_command(Command::new(
        CommandName::ListPlayers,
        String::from("list available MPRIS players"),
        &commands::list_players_cmd,
    ))
    .add_command(Command::new(
        CommandName::Play,
        String::from("send the play command to the preferred player"),
        &commands::play_cmd,
    ))
    .add_command(Command::new(
        CommandName::Pause,
        String::from("send the pause command to the preferred player"),
        &commands::pause_cmd,
    ))
    .add_command(Command::new(
        CommandName::PlayPause,
        String::from("send the play/pause command to the preferred player"),
        &commands::play_pause_cmd,
    ))
}
