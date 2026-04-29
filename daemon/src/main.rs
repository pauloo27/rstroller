use std::{process::exit, sync::RwLock};

use dbus::blocking::{Connection, stdintf::org_freedesktop_dbus::RequestNameReply};
use dbus_crossroads::Crossroads;

mod preferred_player;

struct ServerState {
    preferred_player: RwLock<String>,
}

pub fn main() {
    let conn = Connection::new_session().expect("Failed to create dbus session");
    let request_reply = conn
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

    cr.insert(
        "/cafe/db/code/Rstroller",
        &[iface_token],
        ServerState {
            preferred_player: RwLock::new("".to_string()),
        },
    );

    cr.serve(&conn).expect("Failed to serve");
}
