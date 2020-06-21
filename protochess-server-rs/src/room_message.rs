use crate::client::Client;
use uuid::Uuid;
use crate::client_message::{ClientRequest, RequestGameState};
use std::sync::{ Arc };

pub enum RoomMessage {
    AddClient(Arc<Client>),
    RemoveClient(Uuid),
    External(Uuid, ClientRequest)
}