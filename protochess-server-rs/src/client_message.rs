use serde::{Deserialize, Serialize};
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
        pieces: Vec<Piece>
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
    ChatMessage(String),
    TakeTurn(Turn),
    //Moves from (x,y)
    MovesFrom(u8, u8),
    ListPlayers,
    SwitchLeader(u8),
    StartGame{
        width: u8,
        height: u8,
        tiles: Vec<Tile>,
        pieces: Vec<Piece>
    },
    GameState
}


#[cfg(test)]
mod tests {
    use crate::client_message::ClientRequest;
    use crate::client_message::ClientResponse;
    use crate::client_message::Turn;
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

        let lol = json!(ClientRequest::GameState);
        println!("{}", lol);


        let lol = json!(ClientRequest::SwitchLeader(3));
        println!("{}", lol);


        let lol = json!(ClientRequest::ListPlayers);
        println!("{}", lol);



        let lol = json!(ClientResponse::GameState {
        height:0,
        in_check_kings: None,
        to_move_in_check: false,
        winner: None,
        width: 0,
        last_turn:None,
        pieces:vec![],
        tiles:vec![],
        to_move:0,
        });
        println!("{}", lol);
    }
}