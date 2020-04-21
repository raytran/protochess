extern crate protochess_engine_rs;
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        //let engine = protochess_engine_rs::Engine::from_fen("8/8/8/1q1Q2b1/8/8/8/8 w - - 0 1".parse().unwrap());
        let mut engine = protochess_engine_rs::Engine::default();
        println!("{}",engine.make_move(0,1,0,2,0));

        println!("{}",engine.to_string());
        engine.undo();
        println!("{}",engine.to_string());
        println!("{}",engine.perft(0));
    }
}
