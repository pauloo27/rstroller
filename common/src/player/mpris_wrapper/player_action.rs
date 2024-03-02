use std::time::Duration;

pub enum PlayerAction {
    PlayPause,
    Next,
    Previous,
    Volume(f64),
    Seek(Duration),
    Raise,
    Shuffle(bool),
}
