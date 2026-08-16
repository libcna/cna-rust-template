mod hello_game;
use hello_game::HelloGame;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let smoke_test = args.contains(&"--smoke-test".to_string());
    
    let game = HelloGame::new(smoke_test);
    if let Err(e) = cna::run(game) {
        eprintln!("Game crashed: {:?}", e);
    }
}
