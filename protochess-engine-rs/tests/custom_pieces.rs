extern crate protochess_engine_rs;

#[cfg(test)]
mod custom_pieces {
    #[test]
    fn custom_pieces() {
        //let mut engine = protochess_engine_rs::Engine::default();
        let mut engine = protochess_engine_rs::Engine::custom_pieces();
        println!("{}", engine.to_string());
        engine.make_move(0, 3, 0, 6);
        println!("{}", engine.to_string());
        println!("{}", engine.perft_divide(2));
        println!("{}", engine.to_string());
        engine.make_move(0, 6, 0, 5);
        println!("{}", engine.to_string());
        engine.play_best_move(4);
        println!("{}", engine.to_string());
        println!("{}", engine.to_string());
    }
}
