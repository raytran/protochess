extern crate protochess_engine_rs;

#[cfg(test)]
mod custom_pieces {
    #[test]
    fn custom_pieces() {
        let mut engine = protochess_engine_rs::Engine::custom_pieces();
        println!("{}", engine.to_string());
        println!("{}", engine.make_move(0,3, 5,3));
        println!("{}", engine.to_string());
        println!("{}", engine.make_move(6,6,6,4));
        println!("{}", engine.to_string());
        println!("{}", engine.perft_divide(2));
    }
}
