mod room;
mod client;
mod room_message;
mod client_message;
mod websockets;
mod room_manager;
use std::sync::{ Arc };
use tokio::sync::{RwLock};
use warp::Filter;
use crate::websockets::user_connected;
use crate::room_manager::{RoomManager};

// Clients
pub type Rooms = Arc<RwLock<RoomManager>>;

#[tokio::main]
async fn main() {
    pretty_env_logger::init();
    let rooms = Arc::new(RwLock::new(RoomManager::new()));
    let rooms = warp::any().map(move || rooms.clone());

    // GET /wschess -> websocket upgrade
    let wschess = warp::path("wschess")
        .and(warp::ws())
        .and(rooms.clone())
        .and_then(|ws: warp::ws::Ws, rooms| async move {
            //Hack to get past compiler....
            if false { return Err(warp::reject::not_found()); }
            Ok(ws.on_upgrade(move |socket|
                user_connected(socket, rooms)))
        });

    let assets = warp::fs::dir("./dist");
    let routes = wschess
        .or(assets)
        .or(warp::fs::file("./dist/__app.html"));
    let routes = warp::get().and(routes);
    warp::serve(routes).run(([0, 0, 0, 0], 3030)).await;
}
