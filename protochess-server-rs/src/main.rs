mod room;
mod client;
mod room_message;
mod client_message;
mod websockets;
mod room_manager;
use std::collections::{HashMap};
use std::sync::{ Arc };
use futures::{FutureExt, StreamExt};
use tokio::sync::{RwLock};
use warp::Filter;
use uuid::Uuid;
use crate::room::Room;
use crate::client::Client;
use crate::client_message::{ClientRequest, ClientResponse};
use lazy_static::lazy_static;
use crate::websockets::user_connected;
use crate::room_manager::{RoomManager};

// Clients
pub type Rooms = Arc<RwLock<RoomManager>>;

#[tokio::main]
async fn main() {
    pretty_env_logger::init();
    let rooms = Arc::new(RwLock::new(RoomManager::new()));
    let rooms = warp::any().map(move || rooms.clone());

    // GET /chess -> websocket upgrade
    let chess = warp::path("main")
        .and(warp::ws())
        .and(rooms.clone())
        .and_then(|ws: warp::ws::Ws, rooms| async move {
            if false { return Err(warp::reject::not_found()); }
            Ok(ws.on_upgrade(move |socket|
                user_connected(socket, rooms)))
        });

    // GET / -> index html
    let index = warp::path::end().map(|| warp::reply::html(INDEX_HTML));
    let routes = index.or(chess);
    let routes = warp::get().and(routes);
    warp::serve(routes).run(([0, 0, 0, 0], 3030)).await;
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
        var uri = 'ws://' + location.host + '/main';
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