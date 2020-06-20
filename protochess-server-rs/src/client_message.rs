use serde::{Deserialize, Serialize};
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
pub struct MovementPattern{
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

/// Public facing API
/// Message from the server to client
#[derive(Clone, Serialize, Deserialize, Debug)]
#[serde(tag = "type", content="content")]
pub enum ClientResponse {
    RemovedFromRoom,
    RoomList(Vec<String>),
    CannotOverwriteRoom,
    NoRoomFound,
    ChatMessage {
        from: String,
        content: String
    },
    GameState {
        width: u8,
        height: u8,
        winner: Option<String>,
        to_move: u8,
        to_move_in_check: bool,
        in_check_kings: Option<Vec<Piece>>,
        //from(x,y) to (x,y)
        last_turn: Option<Turn>,
        tiles: Vec<Tile>,
        pieces: Vec<Piece>,
        movement_patterns: HashMap<char, MovementPattern>,
    },
    PlayerList{
        player_num: u8,
        you: String,
        names: Vec<String>
    },
    MovesFrom{
        from: (u8, u8),
        to: Vec<(u8, u8)>
    }
}


/// Message from client to server
#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "type", content ="content")]
pub enum ClientRequest {
    ListRooms,
    CreateRoom{
        room_id: String,
        is_public: bool
    },
    JoinRoom(String),
    LeaveRoom,
    ChatMessage(String),
    TakeTurn(Turn),
    //Moves from (x,y)
    MovesFrom(u8, u8),
    ListPlayers,
    SwitchLeader(u8),
    EditGameState{
        width: u8,
        height: u8,
        tiles: Vec<Tile>,
        pieces: Vec<Piece>,
        movement_patterns: HashMap<char, MovementPattern>,
    },
    GameState
}


#[cfg(test)]
mod tests {
    use crate::client_message::ClientRequest;
    use crate::client_message::ClientResponse;
    use crate::client_message::Turn;
    use std::collections::HashMap;
    use serde_json::json;
    use uuid::Uuid;
    #[test]
    fn serde() {
        let lol = json!(ClientRequest::MovesFrom(0, 1));
        println!("{}", lol);
        let lol = json!(ClientRequest::ChatMessage("lol".to_string()));
        println!("{}", lol);

        let lol = json!(ClientRequest::TakeTurn(Turn{
            from: (0,0),
            to: (0,0),
            promote_to: None
        }));
        println!("{}", lol);

        let lol = json!(ClientRequest::CreateRoom{room_id: "bruh".to_string(), is_public:true});
        println!("{}", lol);


        let lol = json!(ClientRequest::ListRooms);
        println!("{}", lol);


        let lol = json!(ClientRequest::JoinRoom("bruh".to_string()));
        println!("{}", lol);


        let eds = ClientRequest::EditGameState {
            width: 8,
            height: 8,
            tiles: vec![],
            pieces: vec![],
            movement_patterns: HashMap::new(),
        };
        let lol = json!(eds);

        println!("{}", lol);
    }
}