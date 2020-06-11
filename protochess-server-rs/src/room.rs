use tokio::sync::mpsc;
use crate::room_message::RoomMessage;
use crate::client::Client;
use crate::client_message::{ClientRequest, ClientResponse, Piece, Tile};
use uuid::Uuid;
use lazy_static::lazy_static;
use protochess_engine_rs::Move;


lazy_static! {
    static ref MOVEGEN:protochess_engine_rs::MoveGenerator = {
        protochess_engine_rs::MoveGenerator::new()
    };
}


pub struct Room {
    //clients[0] is the leader
    game: protochess_engine_rs::Game,
    clients: Vec<Client>,
    rx: mpsc::UnboundedReceiver<RoomMessage>,
}

impl Room {
    pub fn new(rx: mpsc::UnboundedReceiver<RoomMessage>) -> Room {
        Room{
            game: protochess_engine_rs::Game::default(),
            clients: Vec::new(),
            rx,
        }
    }

    pub async fn run(&mut self){
        while let Some(message) = self.rx.recv().await {
            match message {
                RoomMessage::AddClient(client) => {
                    client.try_send(self.serialize_game());
                    self.clients.push(client);
                    self.broadcast_player_list();
                }
                RoomMessage::RemoveClient(id) => {
                    if let Some(index) = self.clients.iter().position(|x| x.id == id){
                        self.clients.remove(index);
                        //Broadcast the new player list
                        self.broadcast_player_list();
                    }else{
                        eprintln!("no user found at id");
                    }
                }
                RoomMessage::External(requester_id, client_request) => {
                    if let Some(player_num) = self.clients.iter().position(|x| x.id == requester_id){
                        let requester_client = &self.clients[player_num];
                        match client_request {
                            ClientRequest::ChatMessage(m) => {
                                //Send message to other users in the room
                                self.broadcast_except(ClientResponse::ChatMessage {
                                    from: format!("{}", &requester_client.name),
                                    content: format!("{}", m)
                                }, requester_id);
                            }
                            ClientRequest::TakeTurn { from, to } => {
                                //Check if it's this player's turn
                                if player_num as u8 == self.game.get_whos_turn() {
                                    let (x1, y1) = from;
                                    let (x2, y2) = to;
                                    println!("taketurn requested {} {} {} {}", x1, y1, x2, y2);
                                    let move_gen:&protochess_engine_rs::MoveGenerator = &MOVEGEN;
                                    if self.game.make_move(move_gen, x1, y1, x2, y2){
                                        println!("Move successful");
                                        self.broadcast_game_update();
                                    }
                                }

                            }
                            ClientRequest::GameState => {
                                println!("gamestate requested");
                                requester_client.try_send(self.serialize_game());
                            }
                            ClientRequest::StartGame => {
                                println!("start game requested")
                            }
                            ClientRequest::SwitchLeader(new_leader) => {
                                if player_num == 0 && (new_leader as usize) < self.clients.len() {
                                    self.clients.swap(0, new_leader as usize);
                                }
                            }
                            ClientRequest::ListPlayers => {
                                requester_client.try_send(ClientResponse::PlayerList {
                                    player_num: player_num as u8,
                                    you: format!("{}", requester_client.name),
                                    names: self.clients.iter().map(|x| x.name.clone()).collect()
                                })
                            }
                            _ => {}
                        }
                    }else{
                        eprintln!("External room request from unknown client")
                    }
                }
            }

            //Leave if room is empty
            if self.clients.len() == 0 {
                break;
            }
        }
    }

    fn broadcast_game_update(&mut self){
        self.broadcast(self.serialize_game());
    }

    fn serialize_game(&self) -> ClientResponse {
        let width = self.game.current_position.dimensions.width;
        let height = self.game.current_position.dimensions.height;
        let pieces = self.game.current_position.pieces_as_tuples()
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
        let tiles = self.game.current_position.tiles_as_tuples()
            .into_iter()
            .map(|(x, y, tile_type)|{
                Tile{
                    x,
                    y,
                    tile_type
                }
            })
            .collect();
        let to_move = self.game.current_position.whos_turn;
        ClientResponse::GameState {
            width,
            height,
            to_move,
            tiles,
            pieces
        }
    }

    /// Sends a message to everyone except Uuid
    fn broadcast_except(&self, cr: ClientResponse, except_id: Uuid) {
        for client in &self.clients {
            if client.id != except_id {
                client.try_send(cr.clone());
            }
        }
    }

    /// Sends a message to everyone
    fn broadcast(&self, cr: ClientResponse) {
        for client in &self.clients {
            client.try_send(cr.clone());
        }
    }

    fn broadcast_player_list(&self){
        for (i, client) in self.clients.iter().enumerate() {
            client.try_send(ClientResponse::PlayerList {
                player_num: i as u8,
                you: format!("{}", client.name),
                names: self.clients.iter().map(|x| x.name.clone()).collect()
            });
        }
    }
}
