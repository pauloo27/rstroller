use std::process;
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

use common::linux;
use mpris::PlayerFinder;

fn main() {
    if !linux::is_linux() {
        eprintln!("Only linux");
        process::exit(32);
    }

    /*
    TODO: set-player <player> - set the preferred player
    TODO: help - show command usage
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

    hello();
}

fn hello() {
    let (stop_tx, stop_rx) = mpsc::channel();

    let handle = thread::spawn(move || {
        let player = PlayerFinder::new()
            .expect("Couldn't connect to MPRIS")
            .find_active()
            .expect("Couldn't find active player");

        println!("Player found {}", player.bus_name());

        loop {
            if stop_rx.try_recv().is_ok() {
                println!("Stopping event loop");
                break;
            }

            if player.process_events_blocking_for(Duration::from_millis(200)) {
                for event in player.pending_events() {
                    println!("{:#?}", event);
                }
            }
        }
    });

    thread::sleep(Duration::from_secs(10));
    stop_tx.send(()).ok();

    handle.join().ok();
    println!("Event stream ended.");
}
