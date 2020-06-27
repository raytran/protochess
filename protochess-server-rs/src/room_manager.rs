use crate::room::Room;
use serde::{Deserialize, Serialize};
use tokio::sync::{mpsc, RwLock};
use crate::room_message::RoomMessage;
use std::sync::{ Arc };
use std::collections::HashMap;
use crate::client::Client;
use uuid::Uuid;
use tokio::sync::mpsc::{UnboundedSender, UnboundedReceiver};
use crate::client_message::{ClientResponse};
use protochess_common::{GameState, validate_gamestate_request};

/// Holds meta deta for a room
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct RoomInfo {
    room_id: String,
    num_clients: usize,
    is_public: bool,
    editable: bool,
}

struct RoomHandle {
    room_info: RoomInfo,
    room_tx: mpsc::UnboundedSender<RoomMessage>
}

struct RoomDeletionMsg(String);
type BroadcastClients = Arc<RwLock<HashMap<Uuid, Arc<Client>>>>;
type Rooms = Arc<RwLock<HashMap<String, RoomHandle>>>;

/// Controls the creation/deletion/running of each room
pub struct RoomManager{
    broadcast_clients: BroadcastClients,
    rooms: Rooms,
    room_manager_tx: mpsc::UnboundedSender<RoomDeletionMsg>
}

impl RoomManager {
    pub fn new() -> RoomManager {
        let rooms:Rooms = Arc::new(RwLock::new(HashMap::new()));
        let broadcast_clients:BroadcastClients = Arc::new(RwLock::new(HashMap::new()));

        let (tx, mut rx) : (UnboundedSender<RoomDeletionMsg>, UnboundedReceiver<RoomDeletionMsg>) = mpsc::unbounded_channel();
        //Spawn a process responsible for listening to room deletions
        let room_copy = rooms.clone();
        let bc = broadcast_clients.clone();
        tokio::spawn(async move {
            while let Some(room_delete_msg) = rx.recv().await {
                println!("Removing room");
                let mut rms = room_copy.write().await;
                rms.remove(&room_delete_msg.0);
                //Generate new list of rooms
                let mut room_ids = Vec::new();
                for (room_id, room_handler) in rms.iter() {
                    if room_handler.room_info.is_public {
                        room_ids.push(RoomInfo{
                            room_id: room_id.clone(),
                            num_clients: room_handler.room_info.num_clients,
                            is_public: true,
                            editable: room_handler.room_info.editable
                        });
                    }
                }
                //Broadcast removal
                let bcs = bc.read().await;
                for (_id, client) in bcs.iter() {
                    client.try_send(ClientResponse::RoomList(room_ids.clone()))
                }
            }
        });
        RoomManager{
            broadcast_clients,
            rooms,
            room_manager_tx: tx
        }
    }

    /// Registers a client to receive updates on room lists
    /// Sends an initial list
    pub async fn register_broadcast_rooms(&mut self, id: Uuid, client: Arc<Client>){
        let rooms = self.get_public_rooms().await;
        client.try_send(ClientResponse::RoomList(rooms));
        self.broadcast_clients.write().await.insert(id, client);
    }

    /// Unregisters a client to receive updates on room lists
    pub async fn unregister_broadcast_rooms(&mut self, id:&Uuid){
        self.broadcast_clients.write().await.remove(id);
    }

    /// Returns the UnboundedSender<RoomMessage> for the room
    /// if the room_id is not taken and if the init_gamestate is valid
    pub async fn new_room(&mut self, room_id: String,
                          public: bool,
                          editable: bool,
                          init_gamestate: GameState)
        -> Result<UnboundedSender<RoomMessage>,()> {


        {
            if self.rooms.read().await.contains_key(&room_id){
                return Err(());
            }
        }
        let (room_tx, room_rx) = mpsc::unbounded_channel();
        let mut new_room = Room::new(room_rx);

        //Set the initial state for the room
        new_room.editable = editable;
        if let Some((movements, valid_squares, valid_pieces)) =
        validate_gamestate_request(init_gamestate.tiles,
                                         init_gamestate.pieces,
                                         init_gamestate.movement_patterns){
            new_room.game.set_state(movements,
                                    valid_squares,
                                    valid_pieces);
        }else{
            return Err(());
        }

        let room_id_clone = room_id.clone();
        let rmtx = self.room_manager_tx.clone();
        tokio::spawn(async move{
            println!("New room running");
            new_room.run().await;
            println!("Room close request");
            rmtx.send(RoomDeletionMsg(room_id_clone))
        });
        {
            self.rooms.write().await.insert(room_id.clone(), RoomHandle{
                room_info: RoomInfo {
                    room_id: room_id,
                    num_clients: 0,
                    is_public: public,
                    editable
                },
                room_tx: room_tx.clone()
            });
        }
        // room creation successful; broadcast change to listeners
        self.broadcast_rooms().await;

        Ok(room_tx)
    }


    pub async fn add_client_to_room(&self, room_id: &String, client: Arc<Client>) -> Result<UnboundedSender<RoomMessage>, ()> {
        let mut return_val;
        //Have to put block here to let lock go out of scope
        {
            let mut rc = self.rooms.write().await;
            match rc.get_mut(room_id){
                Some(room_handler) => {
                    room_handler.room_info.num_clients += 1;
                    if let Err(_) = room_handler.room_tx.send(RoomMessage::AddClient(client)
                    ){
                        eprintln!("Some error sending to room");
                        return Err(());
                    };
                    return_val = Ok(room_handler.room_tx.clone());
                }
                None => {
                    eprintln!("Tried to access room that doesn't exist");
                    return_val = Err(());
                }
            }
        }
        //Update clients
        self.broadcast_rooms().await;
        return_val
    }

    pub async fn remove_client_from_room(&self, room_id: &String, client_id: &Uuid){
        let mut changed = false;
        if let Some(room_handler) = self.rooms.write().await.get_mut(room_id) {
            room_handler.room_info.num_clients -= 1;
            changed = true;
            if let Err(_) = room_handler.room_tx.send(RoomMessage::RemoveClient(client_id.clone())) {
                eprintln!("Some error sending to room");
            }
        }
        if changed {
            self.broadcast_rooms().await;
        }
    }

    /// Returns a list of public rooms
    pub async fn get_public_rooms(&self) -> Vec<RoomInfo> {
        let mut room_infos = Vec::new();
        let rooms = self.rooms.read().await;
        for (room_id, room_handler) in rooms.iter() {
            if room_handler.room_info.is_public {
                room_infos.push(RoomInfo{
                    room_id: room_id.clone(),
                    num_clients: room_handler.room_info.num_clients,
                    is_public: true,
                    editable:room_handler.room_info.editable
                })
            }
        }
        room_infos
    }

    /// Returns a random new ID that is not yet taken
    pub async fn get_new_id(&self) -> String {
        let mut generator = adjective_adjective_animal::Generator::default();
        let mut id = generator.next().unwrap();
        while self.rooms.read().await.contains_key(&id){
            id = generator.next().unwrap();
        }
        id
    }

    /// Broadcasts public rooms to listeners
    async fn broadcast_rooms(&self){
        let rooms = self.get_public_rooms().await;
        for (_id, c) in self.broadcast_clients.read().await.iter() {
            c.try_send(ClientResponse::RoomList(rooms.clone()));
        }
    }
}