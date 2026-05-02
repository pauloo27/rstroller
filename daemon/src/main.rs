use std::{process::exit, sync::RwLock};

use dbus::blocking::{Connection, stdintf::org_freedesktop_dbus::RequestNameReply};
use dbus_crossroads::Crossroads;
use mpris::{FindingError, PlayerFinder};

mod preferred_player;

struct ServerState {
    preferred_player: RwLock<String>,
}

pub fn main() {
    let server_conn = Connection::new_session().expect("Failed to create dbus server session");
    let request_reply = server_conn
        .request_name("cafe.db.code.Rstroller", false, false, true)
        .expect("Failed to request name");

    if request_reply != RequestNameReply::PrimaryOwner {
        eprintln!("Daemon already running!");
        exit(8);
    }

    /*
    To get:
    busctl --user get-property cafe.db.code.Rstroller /cafe/db/code/Rstroller cafe.db.code.Rstroller PreferredPlayer

    To set:
    busctl --user set-property cafe.db.code.Rstroller /cafe/db/code/Rstroller cafe.db.code.Rstroller PreferredPlayer s "player_name"

    To monitor:
    dbus-monitor --session "type='signal',sender='cafe.db.code.Rstroller'"
    */

    let mut cr = Crossroads::new();

    let iface_token = cr.register(
        "cafe.db.code.Rstroller",
        preferred_player::register_property,
    );

    let initial_player = match PlayerFinder::new()
        .expect("Failed to create dbus client session")
        .find_active()
    {
        Ok(p) => p.bus_name().to_string(),
        Err(FindingError::NoPlayerFound) => "".to_string(),
        Err(e) => panic!("Failed to find active player: {e}"),
    };

    cr.insert(
        "/cafe/db/code/Rstroller",
        &[iface_token],
        ServerState {
            preferred_player: RwLock::new(initial_player),
        },
    );

    cr.serve(&server_conn).expect("Failed to serve");
}
