extern crate protochess_engine_rs;

#[cfg(test)]
mod tests {

    #[test]
    fn perft() {
        //let engine = protochess_engine_rs::Engine::from_fen("8/8/8/1q1Q2b1/8/8/8/8 w - - 0 1".parse().unwrap());
        //let mut engine = protochess_engine_rs::Engine::from_fen("r3k2r/Pppp1ppp/1b3nbN/nP6/BBP1P3/q4N2/Pp1P2PP/R2Q1RK1 w kq - 0 1".parse().unwrap());
        let mut engine = protochess_engine_rs::Engine::default();
        //let mut engine = protochess_engine_rs::Engine::from_fen("8/2p5/3p4/KP5r/1R3p1k/8/4P1P1/8 w - -".parse().unwrap());
        //let mut engine = protochess_engine_rs::Engine::from_fen("r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - ".parse().unwrap());
        //let mut engine = protochess_engine_rs::Engine::from_fen("k7/8/8/2p5/8/8/3P4/7K w - - 0 1".parse().unwrap());

        println!("{}",engine.perft(6));
    }
}
