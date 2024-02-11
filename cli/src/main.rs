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
        "show command usage",
        &commands::help_cmd,
    ))
    .add_command(Command::new(
        CommandName::ListPlayers,
        "list available MPRIS players",
        &commands::list_players_cmd,
    ))
    .add_command(Command::new(
        CommandName::SetPreferredPlayer,
        "set the preferred player",
        &commands::set_preferred_player_cmd,
    ))
    .add_command(Command::new(
        CommandName::Status,
        "return the playback status of the player",
        &commands::status_cmd,
    ))
    .add_command(Command::new(
        CommandName::Play,
        r#"send the "play" command to the player"#,
        &commands::play_cmd,
    ))
    .add_command(Command::new(
        CommandName::Pause,
        r#"send the "pause" command to the player"#,
        &commands::pause_cmd,
    ))
    .add_command(Command::new(
        CommandName::PlayPause,
        r#"send the "play/pause" command to the player"#,
        &commands::play_pause_cmd,
    ))
    .add_command(Command::new(
        CommandName::Stop,
        r#"send the "stop" command to the player"#,
        &commands::stop_cmd,
    ))
    .add_command(Command::new(
        CommandName::Next,
        r#"send the "next" command to the player"#,
        &commands::next_cmd,
    ))
    .add_command(Command::new(
        CommandName::Previous,
        r#"send the "previous" command to the player"#,
        &commands::previous_cmd,
    ))
    .add_command(Command::new(
        CommandName::Metadata,
        "get all or a specific metadata value from the player",
        &commands::metadata_cmd,
    ))
    .add_command(Command::new(
        CommandName::Position,
        "get or set the playback position IN MILLISECONDS of the player, either as a absolute value (1000) or a relative value (eg: 500- or 1200+)",
        &commands::position_cmd,
    ))
    .add_command(Command::new(
        CommandName::Show,
        "show some information about the player",
        &commands::show_cmd,
    ))
    .add_command(Command::new(
        CommandName::Loop,
        r#"get or set the loop status of the player, can be "none", "track" or "playlist""#,
        &commands::loop_cmd,
    ))
    .add_command(Command::new(
        CommandName::Shuffle,
        r#"get or set the loop status of the player, can be "true" or "false""#,
        &commands::shuffle_cmd,
    ))
    .add_command(Command::new(
        CommandName::Volume,
        "get or set the player volume, either as a absolute value (0.5) or a relative value (eg: 0.05- or 0.1+)",
        &commands::volume_cmd,
    ))
    .add_command(Command::new(
        CommandName::Raise,
        r#"send the "raise" command to the player"#,
        &commands::raise_cmd,
    ))
}
