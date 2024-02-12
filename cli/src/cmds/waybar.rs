use crate::cmd::CommandExecContext;
use mpris::DBusError;
use super::CommandName;

use super::utils::exec_player_action_silent;

pub fn waybar_cmd(ctx: CommandExecContext<CommandName>) {
    exec_player_action_silent(&ctx, "polybar", |player| {
        let events = player.events()?;

        show()?;

        for _event in events {
            show()?;
        }

        Ok(())
    });
}

fn show() -> Result<(), DBusError> {
    Ok(())
}
