use tokio::sync::mpsc;
use std::collections::HashMap;
use crate::room_message::RoomMessage;
use crate::client::Client;
use uuid::Uuid;
use crate::client_message::{ClientRequest, ClientResponse};

pub struct Room {
    //clients[0] is the leader
    clients: Vec<Client>,
    rx: mpsc::UnboundedReceiver<RoomMessage>,
}

impl Room {
    pub fn new(rx: mpsc::UnboundedReceiver<RoomMessage>) -> Room {
        Room{
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
                    if let Some(requester_client_index) = self.clients.iter().position(|x| x.id == requester_id){
                        let requester_client = &self.clients[requester_client_index];
                        match client_request {
                            ClientRequest::ChatMessage(m) => {
                                //Send message to other users in the room
                                for client in &self.clients {
                                    if client.id != requester_id {
                                        client.try_send(ClientResponse::ChatMessage {
                                            from: format!("{}", &requester_client.name),
                                            content: format!("{}", m)
                                        });
                                    }
                                }
                            }
                            ClientRequest::TakeTurn { from, to } => {
                                let (x1, y1) = from;
                                let (x2, y2) = to;
                                println!("taketurn requested {} {} {} {}", x1, y1, x2, y2);

                            }
                            ClientRequest::GameState => {
                                println!("gamestate requested");
                                requester_client.try_send(ClientResponse::GameState { width: 0, height: 0 });
                            }
                            ClientRequest::StartGame => {
                                println!("start game requested")
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
}
