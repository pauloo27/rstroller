use clap::{Parser, Subcommand};
use common::linux;
use std::process;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    /// Optional name to operate on
    name: Option<String>,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// does testing things
    Test {
        /// lists test values
        #[arg(short, long)]
        list: bool,
    },
}

fn main() {
    if !linux::is_linux() {
        eprintln!("Only linux");
        process::exit(32);
    }

    let cli = Cli::parse();

    // You can check the value provided by positional arguments, or option arguments
    if let Some(name) = cli.name.as_deref() {
        println!("Value for name: {name}");
    }

    // You can check for the existence of subcommands, and if found use their
    // matches just as you would the top level cmd
    match &cli.command {
        Commands::Test { list } => {
            if *list {
                println!("Printing testing lists...");
            } else {
                println!("Not printing testing lists...");
            }
        }
    }

    /*
    DONE: help - show command usage
    TODO: set-player <player> - set the preferred player
    TODO: show - show some information about the player
    TODO: play - send the "play" command to the player
    TODO: pause - send the "pause" command to the player
    TODO: play-pause - send the "play/pause" command to the player
    TODO: list - list available MPRIS players
    TODO: status - get the playback status of the player
    TODO: stop - send the "stop" command to the player
    TODO: next - send the "next" command to the player
    TODO: previous - send the "previous" command to the player
    TODO: raise - send the "raise" command to the player
    TODO: metadata [key] - get all or a specific metadata value from the player
    TODO: position [value/offset+] - get or set the playback position IN MILLISECONDS of the player, either as a absolute value (1000) or a relative value (eg: 500- or 1200+)
    TODO: loop [none/track/playlist] - get or set the loop status of the player
    TODO: volume [value/offset+] - get or set the player volume, either as a absolute value (0.5) or a relative value (eg: 0.05- or 0.1+)
    TODO: scroll-player <up/down> - cycle the preferred player
    TODO: shuffle [true/false] - get or set the loop status of the player

    TODO: waybar - still haven't figured out yet
    */
}
