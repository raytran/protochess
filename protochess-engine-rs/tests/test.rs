extern crate protochess_engine_rs;

#[cfg(test)]
mod tests {

    #[test]
    fn perft() {
        //let engine = protochess_engine_rs::Engine::from_fen("8/8/8/1q1Q2b1/8/8/8/8 w - - 0 1".parse().unwrap());
        let mut engine = protochess_engine_rs::Engine::from_fen("8/2p5/3p4/KP5r/1R3p1k/8/4P1P1/8 w - - ".parse().unwrap());
        //let mut engine = protochess_engine_rs::Engine::default();
        println!("{}",engine.perft_divide(3));
    }
}
