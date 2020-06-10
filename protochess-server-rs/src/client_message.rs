use serde::{Deserialize, Serialize};

/// Public facing API
/// Message from the server to client
#[derive(Clone, Serialize, Deserialize, Debug)]
pub enum ClientResponse {
    ChatMessage {
        from: String,
        content: String
    },
    GameState{
        width: u8,
        height: u8,
        to_move: u8,
        tiles: Vec<(u8, u8, char)>,
        pieces: Vec<(u8, u8, u8, char)>
    },
    PlayerList{
        names: Vec<String>
    }
}


/// Message from client to server
#[derive(Serialize, Deserialize, Debug)]
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
    }
}