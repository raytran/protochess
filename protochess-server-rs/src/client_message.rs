use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Public facing API
/// Message from the server to client
#[derive(Serialize, Deserialize, Debug)]
pub enum ClientResponse {
    ChatMessage {
        from: String,
        content: String
    },
    GameState{
        width: u8,
        height: u8
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
            from: (2, 2),
            to: (2, 3)
        });
        println!("{}", lol);

        let lol = json!(ClientRequest::GameState);
        println!("{}", lol);


        let lol = json!(ClientRequest::SwitchLeader(3));
        println!("{}", lol);
    }
}