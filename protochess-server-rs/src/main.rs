mod room;
mod client;
mod room_message;
mod client_message;

// #![deny(warnings)]
use std::collections::HashMap;
use std::sync::{
    Arc,
};

use futures::{FutureExt, StreamExt};
use tokio::sync::{mpsc, Mutex};
use warp::ws::WebSocket;
use warp::Filter;
use uuid::Uuid;
use crate::room_message::RoomMessage;
use crate::room::Room;
use crate::client::Client;
use crate::client_message::{ClientRequest, ClientResponse};
use lazy_static::lazy_static;



/// Our state of currently connected users.
///
/// - Key is their id
/// - Value is a sender of `warp::ws::Message`
type RoomChannels = Arc<Mutex<HashMap<String, mpsc::UnboundedSender<RoomMessage>>>>;

#[tokio::main]
async fn main() {
    pretty_env_logger::init();

    // Keep track of all connected users, key is usize, value
    // is a websocket sender.
    let room_channels = Arc::new(Mutex::new(HashMap::new()));
    // Turn our "state" into a new Filter...
    let room_channels = warp::any().map(move || room_channels.clone());

    // GET /chat -> websocket upgrade
    let chat = warp::path("chess")
        // The `ws()` filter will prepare Websocket handshake...
        .and(warp::ws())
        .and(warp::path::param())
        .and(room_channels)
        .and_then(|ws: warp::ws::Ws, room_id: String, room_channels: RoomChannels| async move {
            if room_id.len() > 10 {
                return Err(warp::reject::not_found());
            }
            //Create the room if it does not yet exist
            {
                let mut rc = room_channels.lock().await;
                if !rc.contains_key(&room_id){
                    let (tx, rx) = mpsc::unbounded_channel();
                    let mut new_room = Room::new(rx);
                    let rc_clone = room_channels.clone();
                    let room_id_clone = room_id.clone();
                    tokio::spawn(async move {
                        println!("New room running");
                        new_room.run().await;
                        println!("Room shutting down");
                        //Remove the room from the hashmap
                        rc_clone.lock().await.remove(&room_id_clone);
                    });
                    rc.insert(room_id.clone(), tx);
                }
            }

            // This will call our function if the handshake succeeds.
            Ok(ws.on_upgrade(move |socket| user_connected(socket, room_channels, room_id)))
        });

    // GET / -> index html
    let index = warp::path::end().map(|| warp::reply::html(INDEX_HTML));

    let routes = index.or(chat);

    warp::serve(routes).run(([0, 0, 0, 0], 3030)).await;
}

async fn user_connected(ws: WebSocket, room_channels: RoomChannels, room_id: String){
    let my_id = Uuid::new_v4();
    eprintln!("User connected: {}", my_id);
    //Bind mpsc channel to websocket
    let (web_tx, web_rx) = mpsc::unbounded_channel();
    let (user_ws_tx, mut user_ws_rx) = ws.split();
    tokio::task::spawn(
        web_rx
            .map(|x: ClientResponse| {
                serde_json::to_string(&x).unwrap()
            })
            .map(|x| Ok(warp::ws::Message::text(x)))
            .forward(user_ws_tx)
            .map(|result| {
                if let Err(e) = result {
                    eprintln!("websocket send error: {}", e);
                }
            })
    );
    let mut generator = adjective_adjective_animal::Generator::default();
    //Add client to this room
    {
        let rc = room_channels.lock().await;
        match rc.get(&room_id){
            Some(room_tx) => {
                if let Err(_) = room_tx.send(RoomMessage::AddClient(
                    Client{
                        name: generator.next().unwrap(),
                        id: my_id.clone(),
                        sender: web_tx
                    })
                ){
                    eprintln!("Some error sending to room");
                };
            }
            None => {
                eprintln!("Tried to access room that doesn't exist");
                return;
            }
        }
    }

    //Client websocket loop
    while let Some(result) = user_ws_rx.next().await {
        let msg = match result {
            Ok(msg) => msg,
            Err(e) => {
                eprintln!("websocket error(uid={}): {}", my_id, e);
                break;
            }
        };

        if let Ok(str) = msg.to_str(){
            let res:serde_json::Result<ClientRequest> = serde_json::from_str(str);
            if let Ok(cm) = res {
                if let Some(room_tx) = room_channels.lock().await.get(&room_id){
                    if let Err(_) = room_tx.send(RoomMessage::External(my_id.clone(), cm)) {
                        eprintln!("Client some error sending to room");
                    }
                }else{
                    eprintln!("Client tried sending to room that doesn't exist.");
                    break;
                }
            }
        }
    }

    //If we get here, disconnect
    user_disconnected(my_id, room_channels, room_id).await;

}
async fn user_disconnected(my_id: Uuid, room_channels: RoomChannels, room_id: String) {
    eprintln!("good bye user: {}", my_id);
    //User left, send remove message to room
    if let Some(room_tx) = room_channels.lock().await.get(&room_id){
        if let Err(_) = room_tx.send(RoomMessage::RemoveClient(my_id)) {
            eprintln!("Some error sending to room");
        }
    }
}

static INDEX_HTML: &str = r#"
<!DOCTYPE html>
<html>
    <head>
        <title>Warp Chat</title>
    </head>
    <body>
        <h1>warp chat</h1>
        <div id="chat">
            <p><em>Connecting...</em></p>
        </div>
        <input type="text" id="text" />
        <button type="button" id="send">Send</button>
        <script type="text/javascript">
        var uri = 'ws://' + location.host + '/chess/bruh';
        var ws = new WebSocket(uri);
        function message(data) {
            var line = document.createElement('p');
            line.innerText = data;
            chat.appendChild(line);
        }
        ws.onopen = function() {
            chat.innerHTML = "<p><em>Connected!</em></p>";
        }
        ws.onmessage = function(msg) {
            message(msg.data);
        };
        send.onclick = function() {
            var msg = text.value;
            ws.send(msg);
            text.value = '';
            message('<You>: ' + msg);
        };
        </script>
    </body>
</html>
"#;