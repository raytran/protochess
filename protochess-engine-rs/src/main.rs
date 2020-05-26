#[macro_use] extern crate scan_rules;

pub fn main() {
    let mut engine = protochess_engine_rs::Engine::default();
    //let mut engine = protochess_engine_rs::Engine::from_fen("rnbqkbnr/nnnnnnnn/rrrrrrrr/8/8/8/QQQQQQQQ/RNBQKBNR w KQkq - 0 1".parse().unwrap());
    println!("{}", engine.to_string());
    let mut ply = 0;
    loop {
        engine.play_best_move(4);
        println!("PLY: {} Engine plays: \n", ply);
        ply += 1;
        println!("{}", engine.to_string());
        println!("========================================");

        /*
        readln! {
            // Space-separated ints
            (let x1: u8, let y1: u8, let x2: u8, let y2: u8) => {
                println!("x1 y1 x2 y2: {} {} {} {}", x1, y1, x2, y2);
                engine.make_move(x1, y1, x2, y2);
                println!("{}", engine.to_string());
            }
        }

         */

    }
}