mod utils;

use protochess_engine_rs::Engine;
use protochess_common::{GameState, serialize_game_state, validate_gamestate_request};
use wasm_bindgen::prelude::*;
use wasm_bindgen::__rt::std::rc::Rc;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet() {
    alert("Hello, protochess-engine-wasm!");
}

#[wasm_bindgen]
pub struct Protochess {
    engine: Engine
}

#[wasm_bindgen]
impl Protochess {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Protochess {
        Protochess{
            engine: Engine::default()
        }
    }

    pub fn to_string(&mut self) -> String {
        self.engine.to_string()
    }

    pub fn play_best_move(&mut self, depth: u8) -> bool{
        self.engine.play_best_move(depth)
    }

    pub fn make_move(&mut self, x1: u8, y1: u8, x2: u8, y2: u8) -> bool {
        self.engine.make_move(x1,y1,x2,y2)
    }

    pub fn play_best_move_timeout(&mut self, time: usize) -> i8 {
        let (success, search_depth) = self.engine.play_best_move_timeout(time as u64);
        if !success {
            return -1;
        }else{
            return search_depth as i8;
        }
    }

    pub fn get_best_move_timeout(&mut self, time:usize) -> JsValue {
        let best = self.engine.get_best_move_timeout(time as u64);
        JsValue::from_serde(&best).unwrap()
    }

    pub fn get_state(&self) -> JsValue {
        let game_state = serialize_game_state(&self.engine.current_position);
        JsValue::from_serde(&game_state).unwrap()
    }

    pub fn to_move_in_check(&mut self) -> bool {
        self.engine.to_move_in_check()
    }

    ///True on succcess
    pub fn set_state(&mut self, val: &JsValue) -> bool {
        let request_game_state: GameState = val.into_serde().unwrap();
        if let Some((movements, valid_squares, valid_pieces)) =
        validate_gamestate_request(request_game_state.tiles,
                                   request_game_state.pieces,
                                   request_game_state.movement_patterns){
            self.engine.set_state(movements,
                                valid_squares,
                                valid_pieces);
            return true;
        }
        false
    }

    pub fn moves_from(&mut self, x:u8, y:u8) -> JsValue{
        let moves = self.engine.moves_from(x, y);
        JsValue::from_serde(&moves).unwrap()
    }
}