use std::fmt::Display;
use std::str::FromStr;
use strum_macros::EnumIter;

mod app;
pub use app::App;

pub mod commands;

#[derive(EnumIter, Hash, Eq, PartialEq, Clone)]
pub enum CommandName {
    Help,
    ListPlayers,
    Play,
    Pause,
    PlayPause,
    Stop,
    Next,
    Previous,
}

impl CommandName {
    pub fn value(&self) -> &'static str {
        match self {
            CommandName::Help => "help",
            CommandName::ListPlayers => "list-players",
            CommandName::Play => "play",
            CommandName::Pause => "pause",
            CommandName::PlayPause => "play-pause",
            CommandName::Stop => "stop",
            CommandName::Next => "next",
            CommandName::Previous => "previous",
        }
    }
}

impl FromStr for CommandName {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "help" => Ok(CommandName::Help),
            "list-players" => Ok(CommandName::ListPlayers),
            "play" => Ok(CommandName::Play),
            "pause" => Ok(CommandName::Pause),
            "play-pause" => Ok(CommandName::PlayPause),
            "stop" => Ok(CommandName::Stop),
            "next" => Ok(CommandName::Next),
            "previous" => Ok(CommandName::Previous),
            _ => Err(()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::CommandName;
    use strum::IntoEnumIterator;

    #[test]
    fn test_command_name_from_str() {
        for cmd in CommandName::iter() {
            let value = cmd.value();

            let cmd_from_str: CommandName = value
                .parse()
                .expect(format!("CommandName cannot be parsed from value {value}").as_str());

            assert_eq!(value, cmd_from_str.value());
        }
    }
}

impl Display for CommandName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value())
    }
}

pub struct CommandExecContext<'a> {
    app: &'a App<'a>,
}

pub struct Command<'a> {
    pub name: CommandName,
    pub description: String,
    pub handler: &'a dyn Fn(CommandExecContext),
}

impl<'a> Command<'a> {
    pub fn new(
        name: CommandName,
        description: String,
        handler: &dyn Fn(CommandExecContext),
    ) -> Command {
        Command {
            name,
            description,
            handler,
        }
    }
}
