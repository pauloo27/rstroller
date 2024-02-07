use std::fmt::Display;

#[derive(Hash, Eq, PartialEq, Clone)]
pub enum CommandName {
    Help,
    ListPlayers,
    Play,
    Pause,
    PlayPause,
}

impl CommandName {
    pub fn value(&self) -> &'static str {
        match self {
            CommandName::Help => "help",
            CommandName::ListPlayers => "list-players",
            CommandName::Play => "play",
            CommandName::Pause => "pause",
            CommandName::PlayPause => "play-pause",
        }
    }

    pub fn from(str: &str) -> Option<Self> {
        match str {
            "help" => Some(CommandName::Help),
            "list-players" => Some(CommandName::ListPlayers),
            "play" => Some(CommandName::Play),
            "pause" => Some(CommandName::Pause),
            "play-pause" => Some(CommandName::PlayPause),
            _ => None,
        }
    }
}

impl Display for CommandName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value())
    }
}
