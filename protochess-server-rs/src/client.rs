use uuid::Uuid;
use tokio::sync::mpsc;
use crate::client_message::ClientResponse;

pub struct Client {
    pub(crate) name: String,
    pub(crate) id: Uuid,
    pub(crate) sender: mpsc::UnboundedSender<ClientResponse>
}

impl Client {
    pub fn try_send(&self, cr: ClientResponse){
        if let Err(_) = self.sender.send(cr){
            eprintln!("client send error????");
        }
    }
}


