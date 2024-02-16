use super::CommandName;
use crate::core_definition::CommandExecContext;
use mpris::{DBusError, MetadataValue};
use std::process;

pub fn format_metadata_value(value: &MetadataValue) -> String {
    match value {
        MetadataValue::String(v) => v.into(),
        MetadataValue::I16(v) => v.to_string(),
        MetadataValue::I32(v) => v.to_string(),
        MetadataValue::I64(v) => v.to_string(),
        MetadataValue::U8(v) => v.to_string(),
        MetadataValue::U16(v) => v.to_string(),
        MetadataValue::U32(v) => v.to_string(),
        MetadataValue::U64(v) => v.to_string(),
        MetadataValue::F64(v) => v.to_string(),
        MetadataValue::Bool(v) => v.to_string(),
        MetadataValue::Array(v) => format!("{:?}", v),
        MetadataValue::Map(v) => format!("{:?}", v),
        MetadataValue::Unsupported => "unsupported".into(),
    }
}

pub fn parse_offset(arg: &str) -> Result<f64, String> {
    let arg = &arg[..arg.len() - 1];

    arg.parse::<f64>()
        .map_err(|e| format!("Failed to parse offset: {}", e))
}

pub fn exec_player_action<F>(ctx: &CommandExecContext<CommandName>, action_name: &str, action: F)
where
    F: FnOnce(&mpris::Player) -> Result<(), DBusError>,
{
    exec_player_action_silent(&ctx, action_name, |player| {
        action(player)?;
        println!(
            "Action {action_name} called on player {} ({})",
            player.identity(),
            player.bus_name(),
        );
        Ok(())
    });
}

pub fn exec_player_action_silent<F>(
    ctx: &CommandExecContext<CommandName>,
    action_name: &str,
    action: F,
) where
    F: FnOnce(&mpris::Player) -> Result<(), DBusError>,
{
    let player = match ctx.args.flags.get("player") {
        None => {
            common::player::get_preferred_player_or_first().expect("Failed to get preferred player")
        }
        Some(player_name) => {
            common::player::get_player_by_bus_name(player_name).expect("Failed to get player")
        }
    };

    match player {
        Some(player) => {
            action(&player).expect(format!("Failed to call action {action_name}").as_str());
        }
        None => {
            eprintln!("No player found");
            process::exit(1);
        }
    }
}

pub fn print_if_ok<T>(key: &str, value: Result<T, DBusError>) 
    where T : std::fmt::Debug
{
    if let Ok(value) = value {
        println!("{}: {:?}", key, value);
    }
}
