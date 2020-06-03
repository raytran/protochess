#[macro_use] extern crate scan_rules;

use protochess_engine_rs::{MovementPattern, PieceType};

pub fn main() {
    let mut engine = protochess_engine_rs::Engine::default();
    //let mut engine = protochess_engine_rs::Engine::from_fen("rnbqkbnr/nnnnnnnn/rrrrrrrr/8/8/8/QQQQQQQQ/RNBQKBNR w KQkq - 0 1".parse().unwrap());
    //let mut engine = protochess_engine_rs::Engine::from_fen(("rnbqkbnr/pp4pp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1").parse().unwrap());
    //let mut engine = protochess_engine_rs::Engine::from_fen("r1b3nr/ppqk1Bbp/2pp4/4P1B1/3n4/3P4/PPP2QPP/R4RK1 w - - 1 0".parse().unwrap());
    //let mut engine = protochess_engine_rs::Engine::from_fen("1Q6/5pk1/2p3p1/1pbbN2p/4n2P/8/r5P1/5K2 b - - 0 1".parse().unwrap());
    //let mut engine = protochess_engine_rs::Engine::from_fen("rnbqkbnr/pppppppp/8/8/8/8/8/RNBQKBNR w KQkq - 0 1".parse().unwrap());
    println!("{}", engine.to_string());


    engine.register_piecetype('a',MovementPattern {
        promotion_squares: None,
        promo_vals: None,
        attack_sliding_deltas: vec![vec![]],
        attack_jump_deltas: vec![],
        attack_north: true,
        attack_south: true,
        attack_east: true,
        attack_west: true,
        attack_northeast: false,
        attack_northwest: false,
        attack_southeast: false,
        attack_southwest: false,
        translate_jump_deltas: vec![],
        translate_sliding_deltas: vec![vec![]],
        translate_north: true,
        translate_south: true,
        translate_east: true,
        translate_west: true,
        translate_northeast: true,
        translate_northwest: true,
        translate_southeast: true,
        translate_southwest: true,
    });
    println!("{}", engine.get_zobrist());
    /*
    println!("BASE SCORE: {}", engine.get_score());
    engine.add_piece(0, PieceType::Custom('c'), 0, 1);
    engine.add_piece(0, PieceType::Custom('c'), 1, 1);
    engine.add_piece(0, PieceType::Custom('c'), 2, 1);
    engine.add_piece(0, PieceType::Custom('c'), 3, 1);
    engine.add_piece(0, PieceType::Custom('c'), 4, 1);
    engine.add_piece(0, PieceType::Custom('c'), 5, 1);
    engine.add_piece(0, PieceType::Custom('c'), 6, 1);
    engine.add_piece(0, PieceType::Custom('c'), 7, 1);
    println!("NEW SCORE: {}", engine.get_score());

     */

    println!("{}", engine.to_string());

    let mut ply = 0;
    loop {

        if !engine.play_best_move(8) {
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