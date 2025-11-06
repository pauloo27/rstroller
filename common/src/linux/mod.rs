use std::env::consts;

pub fn is_linux() -> bool {
    consts::OS == "linux"
}
