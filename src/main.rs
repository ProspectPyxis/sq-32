use sq_32::english_draughts::GameEnglishDraughts;
use sq_32::game::Game;
use std::time::Instant;

fn main() {
    // Perft at depth n
    let n = 9;
    let mut game = GameEnglishDraughts::init();
    println!("Running perft at depths up to {}", n);

    for i in 1..=n {
        let now = Instant::now();
        let nodes = sq_32::perft(i, &mut game);
        println!(
            "Positions at depth {}: {} ({:?})",
            i,
            nodes,
            Instant::now().duration_since(now)
        );
    }

    println!("Perft complete");
}
