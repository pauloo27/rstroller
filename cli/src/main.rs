mod cmd;

use cmd::{commands, App, Command, CommandFlag, CommandName};
use once_cell::sync::Lazy;

static PLAYER_FLAG: Lazy<CommandFlag> = Lazy::new(|| {
    CommandFlag::new(
        "player",
        Some("p"),
        "Specify the player for a single command",
        true,
    )
});

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
    .add_flag(&PLAYER_FLAG)
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
        String::from(r#"send the "play" command to the preferred player"#),
        &commands::play_cmd,
    ))
    .add_command(Command::new(
        CommandName::Pause,
        String::from(r#"send the "pause" command to the preferred player"#),
        &commands::pause_cmd,
    ))
    .add_command(Command::new(
        CommandName::PlayPause,
        String::from(r#"send the "play/pause" command to the preferred player"#),
        &commands::play_pause_cmd,
    ))
    .add_command(Command::new(
        CommandName::Stop,
        String::from(r#"send the "stop" command to the preferred player"#),
        &commands::stop_cmd,
    ))
    .add_command(Command::new(
        CommandName::Next,
        String::from(r#"send the "next" command to the preferred player"#),
        &commands::next_cmd,
    ))
    .add_command(Command::new(
        CommandName::Previous,
        String::from(r#"send the "previous" command to the preferred player"#),
        &commands::previous_cmd,
    ))
}
