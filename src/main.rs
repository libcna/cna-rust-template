mod hello_game;
use hello_game::HelloGame;

fn main() {
    let game = HelloGame::new();
    if let Err(e) = cna::run(game) {
        eprintln!("Game crashed: {:?}", e);
    }
}
