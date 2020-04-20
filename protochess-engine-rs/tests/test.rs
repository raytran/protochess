extern crate protochess_engine_rs;
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let engine = protochess_engine_rs::Engine::from_fen("8/8/8/1q1Q2b1/8/8/8/8 w - - 0 1".parse().unwrap());

        println!("{}",engine.perft(0));


    }
}
