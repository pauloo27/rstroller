use std::sync::Arc;

use dbus::MethodErr;
use dbus_crossroads::{IfaceBuilder, PropContext};

use crate::ServerState;

pub fn register_property(b: &mut IfaceBuilder<Arc<ServerState>>) {
    b.property::<String, &str>("PreferredPlayer")
        .get(get_preferred_player)
        .set(set_preferred_player);
}

fn get_preferred_player(
    _ctx: &mut PropContext,
    state: &mut Arc<ServerState>,
) -> Result<String, MethodErr> {
    let preferred_player = state.preferred_player.read();
    match preferred_player {
        Ok(value) => Ok(value.clone()),
        Err(err) => {
            eprint!("{:?}", err);
            Err(MethodErr::failed("failed to acquire lock"))
        }
    }
}

fn set_preferred_player(
    _ctx: &mut PropContext,
    state: &mut Arc<ServerState>,
    new_value: String,
) -> Result<Option<String>, MethodErr> {
    let preferred_player = state.preferred_player.write();
    match preferred_player {
        Ok(mut value) => {
            *value = new_value.clone();
            Ok(Some(new_value))
        }
        Err(err) => {
            eprint!("{:?}", err);
            Err(MethodErr::failed("failed to acquire lock"))
        }
    }
}
