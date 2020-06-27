use serde::{Deserialize, Serialize};
use protochess_engine_rs::{Position, MovementPatternExternal};
use std::collections::HashMap;

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Slides {
    pub north: bool,
    pub east: bool,
    pub south: bool,
    pub west: bool,
    pub northeast: bool,
    pub northwest: bool,
    pub southeast: bool,
    pub southwest: bool,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct MovementPattern {
    pub attack_slides: Slides,
    pub translate_slides: Slides,
    pub attack_jumps:Vec<(i8, i8)>,
    pub translate_jumps: Vec<(i8, i8)>,
    pub attack_slide_deltas: Vec<Vec<(i8, i8)>>,
    pub translate_slide_deltas: Vec<Vec<(i8, i8)>>,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Turn {
    pub promote_to: Option<char>,
    pub from: (u8,u8),
    pub to: (u8, u8)
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Tile {
    pub x: u8,
    pub y: u8,
    pub tile_type: char
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Piece {
    pub owner: u8,
    pub x: u8,
    pub y: u8,
    pub piece_type: char
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct GameState {
    pub width: u8,
    pub height: u8,
    pub to_move: u8,
    pub tiles: Vec<Tile>,
    pub pieces: Vec<Piece>,
    pub movement_patterns: HashMap<char, MovementPattern>,
}

pub fn serialize_game_state(position: &Position) -> GameState {
    let pieces = position.pieces_as_tuples()
        .into_iter()
        .map(|(owner, x, y, piece_type)| {
            Piece{
                owner,
                x,
                y,
                piece_type
            }
        })
        .collect();
    let tiles = position.tiles_as_tuples()
        .into_iter()
        .map(|(x, y, tile_type)|{
            Tile{
                x,
                y,
                tile_type
            }
        })
        .collect();
    let to_move = position.whos_turn;
    let movement_patterns = {
        let mut temp = HashMap::new();
        for (k, v) in position.get_char_movementpattern_map() {
            temp.insert(k, MovementPattern{
                attack_slides: Slides {
                    north: v.attack_north,
                    east: v.attack_east,
                    south: v.attack_south,
                    west: v.attack_west,
                    northeast: v.attack_northeast,
                    northwest: v.attack_northwest,
                    southeast: v.attack_southeast,
                    southwest: v.attack_southwest
                },
                translate_slides: Slides {
                    north: v.translate_north,
                    east: v.translate_east,
                    south: v.translate_south,
                    west: v.translate_west,
                    northeast: v.translate_northeast,
                    northwest: v.translate_northwest,
                    southeast: v.translate_southeast,
                    southwest: v.translate_southwest
                },
                attack_jumps: v.attack_jump_deltas,
                translate_jumps: v.translate_jump_deltas,
                attack_slide_deltas: v.attack_sliding_deltas,
                translate_slide_deltas:v.translate_sliding_deltas
            });
        }
        temp
    };

    GameState {
        width: position.dimensions.width,
        height: position.dimensions.height,
        to_move,
        tiles,
        pieces,
        movement_patterns
    }
}

pub fn validate_gamestate_request(tiles:Vec<Tile>, pieces:Vec<Piece>, movement_patterns:HashMap<char, MovementPattern>)
                                         -> Option<(HashMap<char,MovementPatternExternal>, Vec<(u8, u8)>, Vec<(u8, u8, u8, char)>)>{
    let mut w_has_king = false;
    let mut b_has_king = false;
    for pce in &pieces {
        if pce.piece_type == 'k' {
            if pce.owner == 0 {
                w_has_king = true;
            }else {
                b_has_king = true;
            }
        }
    }

    if w_has_king && b_has_king {
        let valid_squares = tiles
            .into_iter()
            .filter(|sq| sq.x < 16 && sq.y < 16 && (sq.tile_type == 'w' || sq.tile_type == 'b'))
            .map(|sq| (sq.x, sq.y))
            .collect();

        let mut movements = HashMap::new();
        for (key, val) in movement_patterns {
            let piece_type_char = key.to_ascii_lowercase();
            if piece_type_char.is_ascii_alphabetic(){
                movements.insert(piece_type_char, protochess_engine_rs::MovementPatternExternal{
                    promotion_squares: None,
                    promo_vals: None,
                    attack_sliding_deltas: val.attack_slide_deltas,
                    attack_jump_deltas: val.attack_jumps,
                    attack_north: val.attack_slides.north,
                    attack_south: val.attack_slides.south,
                    attack_east: val.attack_slides.east,
                    attack_west: val.attack_slides.west,
                    attack_northeast: val.attack_slides.northeast,
                    attack_northwest: val.attack_slides.northwest,
                    attack_southeast: val.attack_slides.southeast,
                    attack_southwest: val.attack_slides.southwest,
                    translate_jump_deltas: val.translate_jumps,
                    translate_sliding_deltas: val.translate_slide_deltas,
                    translate_north: val.translate_slides.north,
                    translate_south: val.translate_slides.south,
                    translate_east: val.translate_slides.east,
                    translate_west: val.translate_slides.west,
                    translate_northeast: val.translate_slides.northeast,
                    translate_northwest: val.translate_slides.northwest,
                    translate_southeast: val.translate_slides.southeast,
                    translate_southwest: val.translate_slides.southwest
                });
            }
        }

        let pieces = pieces.into_iter().map(|pce| (pce.owner, pce.x, pce.y, pce.piece_type)).collect();

        return Some((movements, valid_squares, pieces));
    }
    None
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use serde_json::json;
    use crate::GameState;
    #[test]
    fn serde() {
        let lol = (json!(GameState{
        width:8,
        height:8,
        to_move:0,
        pieces: vec![],
        tiles: vec![],
        movement_patterns: HashMap::new()
        }));
        println!("{}", lol);
    }
}
