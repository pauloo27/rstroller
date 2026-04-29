use clap::{Parser, Subcommand};
use common::linux;
use std::process;

mod cmds;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct RstrollerCli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// TODO: Set the preferred player
    SetPlayer {
        /// The player to use
        player: String,
    },
    /// TODO: Show information about the player
    Show,
    /// TODO: Send the "play" command to the player
    Play,
    /// TODO: Send the "pause" command to the player
    Pause,
    /// TODO: Send the "play/pause" command to the player
    PlayPause,
    /// TODO: List available MPRIS players
    List,
    /// TODO: Get the playback status of the player
    Status,
    /// TODO: Send the "stop" command to the player
    Stop,
    /// TODO: Send the "next" command to the player
    Next,
    /// TODO: Send the "previous" command to the player
    Previous,
    /// TODO: Send the "raise" command to the player
    Raise,
    /// TODO: Get all or a specific metadata value from the player
    Metadata {
        /// Optional metadata key
        key: Option<String>,
    },
    /// TODO: Get or set the playback position in milliseconds
    Position {
        /// Absolute value (1000) or relative value (500- or 1200+)
        value: Option<String>,
    },
    /// TODO: Get or set the loop status of the player
    Loop {
        /// none, track, or playlist
        status: Option<String>,
    },
    /// TODO: Get or set the player volume
    Volume {
        /// Absolute value (0.5) or relative value (0.05- or 0.1+)
        value: Option<String>,
    },
    /// TODO: Cycle the preferred player
    ScrollPlayer {
        /// up or down
        direction: String,
    },
    /// TODO: Get or set the shuffle status of the player
    Shuffle {
        /// true or false
        value: Option<bool>,
    },
}

fn main() {
    if !linux::is_linux() {
        eprintln!("Only linux");
        process::exit(32);
    }

    let cli = RstrollerCli::parse();

    match &cli.command {
        Commands::SetPlayer { player } => cmds::set_player::run(player),
        Commands::Show => cmds::show::run(),
        Commands::Play => cmds::play::run(),
        Commands::Pause => cmds::pause::run(),
        Commands::PlayPause => cmds::play_pause::run(),
        Commands::List => cmds::list::run(),
        Commands::Status => cmds::status::run(),
        Commands::Stop => cmds::stop::run(),
        Commands::Next => cmds::next::run(),
        Commands::Previous => cmds::previous::run(),
        Commands::Raise => cmds::raise::run(),
        Commands::Metadata { key } => cmds::metadata::run(key.as_deref()),
        Commands::Position { value } => cmds::position::run(value.as_deref()),
        Commands::Loop { status } => cmds::loop_cmd::run(status.as_deref()),
        Commands::Volume { value } => cmds::volume::run(value.as_deref()),
        Commands::ScrollPlayer { direction } => cmds::scroll_player::run(direction),
        Commands::Shuffle { value } => cmds::shuffle::run(*value),
    }
}
