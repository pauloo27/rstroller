use common;

fn main() {
    let player = common::get_player();
    match player {
        Some(player) => println!("Player: {}", player.identity()),
        None => println!("No player found"),
    }
}
