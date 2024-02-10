use args::ParsedArgs;
use std::fmt::Display;
use std::str::FromStr;
use strum_macros::EnumIter;

mod app;
mod args;
pub use app::App;

pub mod commands;

#[derive(EnumIter, Hash, Eq, PartialEq, Clone)]
pub enum CommandName {
    Help,
    ListPlayers,
    Status,
    SetPreferredPlayer,
    Play,
    Pause,
    PlayPause,
    Stop,
    Next,
    Previous,
    Raise,
    Volume,
}

impl CommandName {
    pub fn value(&self) -> &'static str {
        match self {
            CommandName::Help => "help",
            CommandName::ListPlayers => "list-players",
            CommandName::Status => "status",
            CommandName::SetPreferredPlayer => "set-preferred-player",
            CommandName::Play => "play",
            CommandName::Pause => "pause",
            CommandName::PlayPause => "play-pause",
            CommandName::Stop => "stop",
            CommandName::Next => "next",
            CommandName::Previous => "previous",
            CommandName::Raise => "raise",
            CommandName::Volume => "volume",
        }
    }
}

impl FromStr for CommandName {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "help" => Ok(CommandName::Help),
            "list-players" => Ok(CommandName::ListPlayers),
            "status" => Ok(CommandName::Status),
            "play" => Ok(CommandName::Play),
            "pause" => Ok(CommandName::Pause),
            "play-pause" => Ok(CommandName::PlayPause),
            "stop" => Ok(CommandName::Stop),
            "next" => Ok(CommandName::Next),
            "previous" => Ok(CommandName::Previous),
            "set-preferred-player" => Ok(CommandName::SetPreferredPlayer),
            "raise" => Ok(CommandName::Raise),
            "volume" => Ok(CommandName::Volume),
            _ => Err(()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::CommandName;
    use std::collections::HashMap;
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

    #[test]
    fn test_no_duplicates() {
        let mut values = HashMap::new();
        for cmd in CommandName::iter() {
            let value = cmd.value();

            if values.contains_key(value) {
                panic!("Duplicate value: {}", value);
            }
            values.insert(value, true);
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
    args: ParsedArgs,
}

#[derive(Debug)]
pub struct CommandFlag {
    pub name: &'static str,
    pub short_name: Option<&'static str>,
    pub description: &'static str,
    pub has_value: bool,
}

impl CommandFlag {
    pub fn new(
        name: &'static str,
        short_name: Option<&'static str>,
        description: &'static str,
        has_value: bool,
    ) -> Self {
        CommandFlag {
            name,
            short_name,
            description,
            has_value,
        }
    }
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
