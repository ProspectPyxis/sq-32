use sq_32::english_draughts::GameEnglishDraughts;
use sq_32::game::Game;
use std::time::Instant;

fn main() {
    // Perft at depth n
    let n = 9;
    let mut game = GameEnglishDraughts::init();
    println!("Running perft at depths up to {}", n);

    let mut all_nps: Vec<f64> = Vec::new();

    for i in 1..=n {
        let then = Instant::now();
        let nodes = sq_32::perft(i, &mut game);
        let duration = Instant::now().duration_since(then);
        let nps = (nodes as f64 / duration.as_secs_f64()).floor();
        println!(
            "Positions at depth {}: {} ({}ms, {} nodes/sec)",
            i,
            nodes,
            duration.as_micros(),
            nps,
        );
        all_nps.push(nps);
    }

    let first_nps = *all_nps.first().unwrap();
    println!("Perft complete");
    println!(
        "Average nodes/sec: {}",
        all_nps
            .iter()
            .fold(first_nps, |acc, x| (acc + x) / 2.0)
            .round(),
    );
}
