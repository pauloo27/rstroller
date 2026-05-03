use std::{process::exit, sync::Arc, thread};

use dbus::blocking::{Connection, stdintf::org_freedesktop_dbus::RequestNameReply};
use dbus_crossroads::Crossroads;

use crate::server_state::ServerState;

mod dbus_interface;
mod server_state;

const DBUS_PATH: &str = "/cafe/db/code/Rstroller";

/*
To get:
busctl --user get-property cafe.db.code.Rstroller /cafe/db/code/Rstroller cafe.db.code.Rstroller PreferredPlayer

To set:
busctl --user set-property cafe.db.code.Rstroller /cafe/db/code/Rstroller cafe.db.code.Rstroller PreferredPlayer s "player_name"

To monitor:
dbus-monitor --session "type='signal',sender='cafe.db.code.Rstroller'"
*/

pub fn main() {
    let server_conn = Connection::new_session().expect("Failed to create dbus server session");
    let request_reply = server_conn
        .request_name("cafe.db.code.Rstroller", false, false, true)
        .expect("Failed to request name");

    if request_reply != RequestNameReply::PrimaryOwner {
        eprintln!("Daemon already running!");
        exit(8);
    }

    let mut cr = Crossroads::new();

    let iface_token = cr.register("cafe.db.code.Rstroller", dbus_interface::register_property);

    let state = ServerState::load_initial().expect("Failed to load initial player");
    let state = Arc::new(state);

    cr.insert(DBUS_PATH, &[iface_token], state.clone());

    thread::spawn(move || {
        state.watch_for_changes();
    });

    println!("Starting dbus server...");
    cr.serve(&server_conn).expect("Failed to serve");
}
