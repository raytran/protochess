extern crate protochess_engine_rs;

#[cfg(test)]
mod custom_pieces {
    use protochess_engine_rs::{MovementPattern, PieceType};

    #[test]
    fn custom_pieces() {
        let mut engine = protochess_engine_rs::Engine::default();

        engine.register_piecetype(0,'a',MovementPattern {
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
        engine.add_piece(0, PieceType::Custom('a'), 0, 3);
        println!("{}", engine.to_string());
        engine.make_move(5, 1, 5, 2);
        engine.make_move(2, 6, 2, 5);
        println!("{}", engine.to_string());
        println!("{}", engine.perft_divide(2));
        engine.make_move(0, 3, 0, 6);
        println!("{}", engine.to_string());
    }
}
