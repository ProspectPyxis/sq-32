use sq_32::english_draughts::GameEnglishDraughts;
use sq_32::game::Game;
use std::time::Instant;

fn main() {
    // Perft at depth n...
    let n = 13;
    // ...m times
    let m = 1;

    let mut avg_nps: Vec<f64> = Vec::new();

    for l in 1..=m {
        let mut game = GameEnglishDraughts::init();
        println!("Running perft at depths up to {}", n);

        let mut all_nps: Vec<f64> = Vec::new();
        let mut mc = 0;

        for i in 1..=n {
            let then = Instant::now();
            let (nodes, count) = sq_32::perft(i, &mut game);
            let duration = Instant::now().duration_since(then);
            let nps = (nodes as f64 / duration.as_secs_f64()).floor();
            println!(
                "Positions at depth {}: {} ({} ms, {} nodes/sec)",
                i,
                nodes,
                duration.as_micros(),
                nps,
            );
            mc = mc.max(count);
            all_nps.push(nps);
        }

        let total_nps = (all_nps.iter().fold(0.0, |acc, x| acc + x) / all_nps.len() as f64).floor();

        println!("Perft {} complete", l);
        println!("Average nodes/sec: {}", total_nps);
        println!("Maximum move count in position: {}", mc);
        avg_nps.push(total_nps);
    }

    println!("\nAll perft runs complete");
    println!(
        "Final average nodes/sec across {} runs: {}",
        m,
        (avg_nps.iter().fold(0.0, |acc, x| acc + x) / avg_nps.len() as f64).floor()
    );
}
