use serde::{Deserialize, Serialize};
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
    GameState{
        width: u8,
        height: u8,
        to_move: u8,
        tiles: Vec<Tile>,
        pieces: Vec<Piece>
    },
    PlayerList{
        player_num: u8,
        you: String,
        names: Vec<String>
    }
}


/// Message from client to server
#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "type", content ="content")]
pub enum ClientRequest {
    ChatMessage(String),
    TakeTurn{
        from: (u8, u8),
        to: (u8, u8)
    },
    ListPlayers,
    SwitchLeader(u8),
    StartGame,
    GameState
}


#[cfg(test)]
mod tests {
    use crate::client_message::ClientRequest;
    use serde_json::json;
    use uuid::Uuid;
    #[test]
    fn serde() {
        let lol = json!(ClientRequest::ChatMessage("lol".to_string()));
        println!("{}", lol);

        let lol = json!(ClientRequest::TakeTurn{
            from: (0, 2),
            to: (0, 3)
        });
        println!("{}", lol);

        let lol = json!(ClientRequest::GameState);
        println!("{}", lol);


        let lol = json!(ClientRequest::SwitchLeader(3));
        println!("{}", lol);


        let lol = json!(ClientRequest::ListPlayers);
        println!("{}", lol);
    }
}