extern crate protochess_engine_rs;

#[cfg(test)]
mod custom_pieces {
    #[test]
    fn custom_pieces() {
        let mut engine = protochess_engine_rs::Engine::default();
        println!("{}", engine.to_string());
    }
}
