use crate::room::Room;
use tokio::sync::{mpsc, RwLock, Mutex};
use crate::room_message::RoomMessage;
use std::sync::{ Arc };
use std::collections::HashMap;
use crate::client::Client;
use uuid::Uuid;
use tokio::sync::mpsc::{UnboundedSender, UnboundedReceiver};
use crate::client_message::ClientResponse;

/// Holds meta deta for a room
struct RoomHandle {
    is_public: bool,
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

        let (mut tx, mut rx) : (UnboundedSender<RoomDeletionMsg>, UnboundedReceiver<RoomDeletionMsg>) = mpsc::unbounded_channel();
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
                    if room_handler.is_public {
                        room_ids.push(room_id.clone());
                    }
                }
                //Broadcast removal
                let bcs = bc.read().await;
                for (id, client) in bcs.iter() {
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
        let rooms = self.get_public_room_ids().await;
        client.try_send(ClientResponse::RoomList(rooms));
        self.broadcast_clients.write().await.insert(id, client);
    }

    /// Unregisters a client to receive updates on room lists
    pub async fn unregister_broadcast_rooms(&mut self, id:&Uuid){
        self.broadcast_clients.write().await.remove(id);
    }

    /// Returns the UnboundedSender<RoomMessage> for the room
    pub async fn new_room(&mut self, room_id: String, public: bool) -> Result<UnboundedSender<RoomMessage>,()> {
        {
            if self.rooms.read().await.contains_key(&room_id){
                return Err(());
            }
        }
        let (room_tx, room_rx) = mpsc::unbounded_channel();
        let mut new_room = Room::new(room_rx);
        let room_id_clone = room_id.clone();
        let rmtx = self.room_manager_tx.clone();
        tokio::spawn(async move{
            println!("New room running");
            new_room.run().await;
            println!("Room close request");
            rmtx.send(RoomDeletionMsg(room_id_clone))
        });
        self.rooms.write().await.insert(room_id, RoomHandle{
            is_public: public,
            room_tx: room_tx.clone()
        });
        // room creation successful; broadcast change to listeners
        self.broadcast_rooms().await;

        Ok(room_tx)
   }


    pub async fn add_client_to_room(&self, room_id: &String, client: Arc<Client>) -> Result<UnboundedSender<RoomMessage>, ()> {
        let rc = self.rooms.read().await;
        match rc.get(room_id){
            Some(room_handler) => {
                if let Err(_) = room_handler.room_tx.send(RoomMessage::AddClient(client)
                ){
                    eprintln!("Some error sending to room");
                    return Err(());
                };
                return Ok(room_handler.room_tx.clone());
            }
            None => {
                eprintln!("Tried to access room that doesn't exist");
                return Err(());
            }
        }
    }

    pub async fn remove_client_from_room(&self, room_id: &String, client_id: &Uuid){
        if let Some(room_handler) = self.rooms.read().await.get(room_id) {
            if let Err(_) = room_handler.room_tx.send(RoomMessage::RemoveClient(client_id.clone())) {
                eprintln!("Some error sending to room");
            }
        }
    }

    /// Returns a list of public rooms
    pub async fn get_public_room_ids(&self) -> Vec<String> {
        let mut room_ids = Vec::new();
        let rooms = self.rooms.read().await;
        for (room_id, room_handler) in rooms.iter() {
            if room_handler.is_public {
                room_ids.push(room_id.clone());
            }
        }
        room_ids
    }

    /// Broadcasts public rooms to listeners
    async fn broadcast_rooms(&self){
        let rooms = self.get_public_room_ids().await;
        for (id, c) in self.broadcast_clients.read().await.iter() {
            c.try_send(ClientResponse::RoomList(rooms.clone()));
        }
    }
}