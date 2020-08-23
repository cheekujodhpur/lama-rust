use lama_rust::player::{Player};
use lama_rust::game::{Game};

fn main() {
    let mut game = Game::new();
    let player = Player::new(String::from("cheeku"));
    game.add_player(player);
    game.print_players();
    // println!( "{}", player );
}
