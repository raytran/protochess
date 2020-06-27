use serde::{Deserialize, Serialize};
use crate::room_manager::RoomInfo;
use protochess_common::{GameState, Piece, Turn};

/// Public facing API
/// Message from the server to client
#[derive(Clone, Serialize, Deserialize, Debug)]
#[serde(tag = "type", content="content")]
pub enum ClientResponse {
    RoomCreateSuccess(String),
    RemovedFromRoom,
    RoomList(Vec<RoomInfo>),
    CannotOverwriteRoom,
    NoRoomFound,
    ChatMessage {
        from: String,
        content: String
    },
    GameInfo {
        editable: bool,
        winner: Option<String>,
        to_move_in_check: bool,
        in_check_kings: Option<Vec<Piece>>,
        last_turn: Option<Turn>,
        state: GameState
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
        allow_edits: bool,
        is_public: bool,
        init_game_state: GameState
    },
    JoinRoom(String),
    LeaveRoom,
    ChatMessage(String),
    TakeTurn(Turn),
    //Moves from (x,y)
    MovesFrom(u8, u8),
    ListPlayers,
    SwitchLeader(u8),
    EditGameState(GameState),
    DisableEdits,
    GameState
}


#[cfg(test)]
mod tests {
    use crate::client_message::{ClientRequest };
    use crate::client_message::ClientResponse;
    use crate::client_message::Turn;
    use std::collections::HashMap;
    use serde_json::json;
    use uuid::Uuid;
    #[test]
    fn serde() {
        /*
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

        let rgs = RequestGameState{
            width: 0,
            height: 0,
            tiles: vec![],
            pieces: vec![],
            movement_patterns: HashMap::new()
        };


        let lol = json!(ClientRequest::ListRooms);
        println!("{}", lol);


        let lol = json!(ClientRequest::JoinRoom("bruh".to_string()));
        println!("{}", lol);


        let eds = ClientRequest::LeaveRoom;
        let lol = json!(eds);

        println!("{}", lol);

         */
    }
}