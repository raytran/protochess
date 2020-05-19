extern crate protochess_engine_rs;

#[cfg(test)]
mod tests {

    #[test]
    fn perft() {
        //let engine = protochess_engine_rs::Engine::from_fen("8/8/8/1q1Q2b1/8/8/8/8 w - - 0 1".parse().unwrap());
        let mut engine = protochess_engine_rs::Engine::from_fen("r3k2r/Pppp1ppp/1b3nbN/nP6/BBP1P3/q4N2/Pp1P2PP/R2Q1RK1 w kq - 0 1".parse().unwrap());
        //let mut engine = protochess_engine_rs::Engine::default();

        println!("{}",engine.perft(2));
    }
}
