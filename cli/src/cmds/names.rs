use std::fmt::Display;
use std::str::FromStr;
use strum_macros::EnumIter;

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
    Metadata,
    Position,
    Loop,
    Shuffle,
    Show,
    Waybar,
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
            CommandName::Metadata => "metadata",
            CommandName::Position => "position",
            CommandName::Loop => "loop",
            CommandName::Shuffle => "shuffle",
            CommandName::Show => "show",
            CommandName::Waybar => "waybar",
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
            "position" => Ok(CommandName::Position),
            "volume" => Ok(CommandName::Volume),
            "metadata" => Ok(CommandName::Metadata),
            "waybar" => Ok(CommandName::Waybar),
            "loop" => Ok(CommandName::Loop),
            "shuffle" => Ok(CommandName::Shuffle),
            "show" => Ok(CommandName::Show),
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