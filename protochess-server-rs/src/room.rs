use tokio::sync::mpsc;
use crate::room_message::RoomMessage;
use crate::client::Client;
use crate::client_message::{ClientRequest, ClientResponse};
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
    position: protochess_engine_rs::Position,
    clients: Vec<Client>,
    rx: mpsc::UnboundedReceiver<RoomMessage>,
}

impl Room {
    pub fn new(rx: mpsc::UnboundedReceiver<RoomMessage>) -> Room {
        Room{
            position: protochess_engine_rs::Position::default(),
            clients: Vec::new(),
            rx,
        }
    }

    pub async fn run(&mut self){
        while let Some(message) = self.rx.recv().await {
            match message {
                RoomMessage::AddClient(client) => {
                    self.clients.push(client);
                }
                RoomMessage::RemoveClient(id) => {
                    if let Some(index) = self.clients.iter().position(|x| x.id == id){
                        self.clients.remove(index);
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
                                let (x1, y1) = from;
                                let (x2, y2) = to;
                                println!("taketurn requested {} {} {} {}", x1, y1, x2, y2);
                                for move_ in MOVEGEN.get_pseudo_moves(&mut self.position){
                                    println!("{}", move_);
                                }

                            }
                            ClientRequest::GameState => {
                                println!("gamestate requested");
                                requester_client.try_send(ClientResponse::GameState { width: 0, height: 0 });
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
}
