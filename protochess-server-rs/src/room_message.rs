use crate::client::Client;
use uuid::Uuid;
use crate::client_message::ClientRequest;

pub enum RoomMessage {
    AddClient(Client),
    RemoveClient(Uuid),
    External(Uuid, ClientRequest)
}