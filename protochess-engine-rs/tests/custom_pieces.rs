extern crate protochess_engine_rs;

#[cfg(test)]
mod custom_pieces {
    use protochess_engine_rs::{MovementPatternExternal, PieceType};

    #[test]
    fn custom_pieces() {
        let mut pos = protochess_engine_rs::Position::default();
        pos.register_piecetype('c',MovementPatternExternal {
            promotion_squares: None,
            promo_vals: None,
            attack_sliding_deltas: vec![vec![(1,1),(2,2)]],
            attack_jump_deltas: vec![],
            attack_north: false,
            attack_south: false,
            attack_east: false,
            attack_west: false,
            attack_northeast: false,
            attack_northwest: false,
            attack_southeast: false,
            attack_southwest: false,
            translate_jump_deltas: vec![],
            translate_sliding_deltas: vec![vec![(1,1),(2,2)]],
            translate_north: false,
            translate_south: false,
            translate_east: false,
            translate_west: false,
            translate_northeast: false,
            translate_northwest: false,
            translate_southeast: false,
            translate_southwest: false
        });
        pos.register_piecetype('a',MovementPatternExternal {
            promotion_squares: None,
            promo_vals: None,
            attack_sliding_deltas: vec![vec![(1,1),(2,2)]],
            attack_jump_deltas: vec![],
            attack_north: false,
            attack_south: false,
            attack_east: false,
            attack_west: false,
            attack_northeast: false,
            attack_northwest: false,
            attack_southeast: false,
            attack_southwest: false,
            translate_jump_deltas: vec![],
            translate_sliding_deltas: vec![vec![(1,1),(2,2)]],
            translate_north: false,
            translate_south: false,
            translate_east: false,
            translate_west: false,
            translate_northeast: false,
            translate_northwest: false,
            translate_southeast: false,
            translate_southwest: false
        });

        for thing in pos.get_char_movementpattern_map(){
            println!("{:?}", thing);
        }


        let mut engine = protochess_engine_rs::Engine::default();


        /*
        engine.register_piecetype('a',MovementPattern {
            promotion_squares: None,
            promo_vals: None,
            attack_sliding_deltas: vec![vec![(1,1),(2,2)]],
            attack_jump_deltas: vec![],
            attack_north: false,
            attack_south: false,
            attack_east: false,
            attack_west: false,
            attack_northeast: false,
            attack_northwest: false,
            attack_southeast: false,
            attack_southwest: false,
            translate_jump_deltas: vec![],
            translate_sliding_deltas: vec![vec![(1,1),(2,2)]],
            translate_north: false,
            translate_south: false,
            translate_east: false,
            translate_west: false,
            translate_northeast: false,
            translate_northwest: false,
            translate_southeast: false,
            translate_southwest: false
        });

         */
        println!("{}", engine.get_zobrist());
        println!("BASE SCORE: {}", engine.get_score());
        engine.add_piece(0, PieceType::Custom('a'), 0, 3);
        println!("NEW SCORE: {}", engine.get_score());
        println!("{}", engine.to_string());


        let mut ply = 0;
        engine.play_best_move(1);
        ply += 1;
        println!("PLY: {} Engine plays: \n", ply);
        println!("{}", engine.to_string());
        println!("========================================");
    }

}
