extern crate protochess_engine_rs;

#[cfg(test)]
mod fen {
    #[test]
    fn starting_pos() {
        let mut engine = protochess_engine_rs::Engine::default();
        assert_eq!(engine.perft(1), 20);
        assert_eq!(engine.perft(2), 400);
        assert_eq!(engine.perft(3), 8902);
        assert_eq!(engine.perft(4), 197281);
        assert_eq!(engine.perft(5), 4865609);
    }

    #[test]
    fn kiwipete(){
        let mut engine = protochess_engine_rs::Engine::from_fen("r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - ".parse().unwrap());
        assert_eq!(engine.perft(1), 48);
        assert_eq!(engine.perft(2), 2039);
        assert_eq!(engine.perft(3), 97862);
        assert_eq!(engine.perft(4), 4085603);
        assert_eq!(engine.perft(5), 193690690);
    }

    #[test]
    fn pos3(){
        let mut engine = protochess_engine_rs::Engine::from_fen("8/2p5/3p4/KP5r/1R3p1k/8/4P1P1/8 w - - ".parse().unwrap());
        assert_eq!(engine.perft(1), 14);
        assert_eq!(engine.perft(2), 191);
        assert_eq!(engine.perft(3), 2812);
        assert_eq!(engine.perft(4), 43238);
        assert_eq!(engine.perft(5), 674624);
    }

    #[test]
    fn pos4(){
        let mut engine = protochess_engine_rs::Engine::from_fen("r3k2r/Pppp1ppp/1b3nbN/nP6/BBP1P3/q4N2/Pp1P2PP/R2Q1RK1 w kq - 0 1".parse().unwrap());
        assert_eq!(engine.perft(1), 6);
        assert_eq!(engine.perft(2), 264);
        assert_eq!(engine.perft(3), 9467);
        assert_eq!(engine.perft(4), 422333);
        assert_eq!(engine.perft(5), 15833292);
    }

    #[test]
    fn pos5(){
        let mut engine = protochess_engine_rs::Engine::from_fen("rnbq1k1r/pp1Pbppp/2p5/8/2B5/8/PPP1NnPP/RNBQK2R w KQ - 1 8  ".parse().unwrap());
        assert_eq!(engine.perft(1), 44);
        assert_eq!(engine.perft(2), 1486);
        assert_eq!(engine.perft(3), 62379);
        assert_eq!(engine.perft(4), 2103487);
        assert_eq!(engine.perft(5), 89941194);
    }

    #[test]
    fn pos6(){
        let mut engine = protochess_engine_rs::Engine::from_fen("r4rk1/1pp1qppp/p1np1n2/2b1p1B1/2B1P1b1/P1NP1N2/1PP1QPPP/R4RK1 w - - 0 10 ".parse().unwrap());
        assert_eq!(engine.perft(1), 46);
        assert_eq!(engine.perft(2), 2079);
        assert_eq!(engine.perft(3), 89890);
        assert_eq!(engine.perft(4), 3894594);
        assert_eq!(engine.perft(5), 164075551);
    }

}
