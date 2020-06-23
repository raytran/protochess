#[macro_use] extern crate scan_rules;

pub fn main() {
    let mut engine = protochess_engine_rs::Engine::default();
    //let mut engine = protochess_engine_rs::Engine::from_fen("rnbqkbnr/nnnnnnnn/rrrrrrrr/8/8/8/QQQQQQQQ/RNBQKBNR w KQkq - 0 1".parse().unwrap());
    //let mut engine = protochess_engine_rs::Engine::from_fen(("rnbqkbnr/pp4pp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1").parse().unwrap());
    //let mut engine = protochess_engine_rs::Engine::from_fen("r1b3nr/ppqk1Bbp/2pp4/4P1B1/3n4/3P4/PPP2QPP/R4RK1 w - - 1 0".parse().unwrap());
    //let mut engine = protochess_engine_rs::Engine::from_fen("1Q6/5pk1/2p3p1/1pbbN2p/4n2P/8/r5P1/5K2 b - - 0 1".parse().unwrap());
    //let mut engine = protochess_engine_rs::Engine::from_fen("rnbqkbnr/pppppppp/8/8/8/8/8/RNBQKBNR w KQkq - 0 1".parse().unwrap());
    println!("{}", engine.to_string());

    let mut ply = 0;
    loop {

        if !engine.play_best_move_timeout(4).0 {
            break;
        }
        ply += 1;
        println!("PLY: {} Engine plays: \n", ply);
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