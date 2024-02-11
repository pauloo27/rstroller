mod cmd;

use cmd::{cmds, App, Command, CommandFlag, CommandName};
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
        &cmds::help_cmd,
    ))
    .add_command(Command::new(
        CommandName::ListPlayers,
        "list available MPRIS players",
        &cmds::list_players_cmd,
    ))
    .add_command(Command::new_with_usage(
        CommandName::SetPreferredPlayer,
        "<player>",
        "set the preferred player",
        &cmds::set_preferred_player_cmd,
    ))
    .add_command(Command::new(
        CommandName::Status,
        "get the playback status of the player",
        &cmds::status_cmd,
    ))
    .add_command(Command::new(
        CommandName::Play,
        r#"send the "play" command to the player"#,
        &cmds::play_cmd,
    ))
    .add_command(Command::new(
        CommandName::Pause,
        r#"send the "pause" command to the player"#,
        &cmds::pause_cmd,
    ))
    .add_command(Command::new(
        CommandName::PlayPause,
        r#"send the "play/pause" command to the player"#,
        &cmds::play_pause_cmd,
    ))
    .add_command(Command::new(
        CommandName::Stop,
        r#"send the "stop" command to the player"#,
        &cmds::stop_cmd,
    ))
    .add_command(Command::new(
        CommandName::Next,
        r#"send the "next" command to the player"#,
        &cmds::next_cmd,
    ))
    .add_command(Command::new(
        CommandName::Previous,
        r#"send the "previous" command to the player"#,
        &cmds::previous_cmd,
    ))
    .add_command(Command::new_with_usage(
        CommandName::Metadata,
        "[key]",
        "get all or a specific metadata value from the player",
        &cmds::metadata_cmd,
    ))
    .add_command(Command::new_with_usage(
        CommandName::Position,
        "[value/offset+]",
        "get or set the playback position IN MILLISECONDS of the player, either as a absolute value (1000) or a relative value (eg: 500- or 1200+)",
        &cmds::position_cmd,
    ))
    .add_command(Command::new(
        CommandName::Show,
        "show some information about the player",
        &cmds::show_cmd,
    ))
    .add_command(Command::new_with_usage(
        CommandName::Loop,
        "[none/track/playlist]",
        r#"get or set the loop status of the player"#,
        &cmds::loop_cmd,
    ))
    .add_command(Command::new_with_usage(
        CommandName::Shuffle,
        "[true/false]",
        r#"get or set the loop status of the player"#,
        &cmds::shuffle_cmd,
    ))
    .add_command(Command::new_with_usage(
        CommandName::Volume,
        "[value/offset+]",
        "get or set the player volume, either as a absolute value (0.5) or a relative value (eg: 0.05- or 0.1+)",
        &cmds::volume_cmd,
    ))
    .add_command(Command::new(
        CommandName::Raise,
        r#"send the "raise" command to the player"#,
        &cmds::raise_cmd,
    ))
    .add_command(Command::new(
        CommandName::Waybar,
        "still haven't figured out yet",
        &cmds::waybar_cmd,
    ))
}
