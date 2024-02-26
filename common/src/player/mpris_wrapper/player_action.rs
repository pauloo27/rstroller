pub enum PlayerAction {
    PlayPause,
    Next,
    Previous,
    Volume(f64),
    Raise,
    Shuffle(bool),
}
