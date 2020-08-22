mod player;
use player::{Player};

fn main() {
    let player = Player::new(String::from("cheeku"));
    println!( "{}", player );
}
