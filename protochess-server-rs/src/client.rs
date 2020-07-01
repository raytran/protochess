use uuid::Uuid;
use tokio::sync::mpsc;
use crate::client_message::ClientResponse;

pub struct Client {
    pub(crate) name: String,
    pub(crate) id: Uuid,
    pub(crate) sender: mpsc::UnboundedSender<Result<warp::ws::Message, warp::Error>>
}

impl Client {
    pub fn try_send(&self, cr: ClientResponse){
        if let Ok(text) = serde_json::to_string(&cr){
            let msg = Ok(warp::ws::Message::text(text));
            if let Err(_) = self.sender.send(msg){
                eprintln!("client send error????");
            }
        }
    }
}


