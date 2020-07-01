use tokio::sync::{mpsc};
use futures::{FutureExt, StreamExt};
use warp::ws::WebSocket;
use std::sync::{ Arc };
use std::sync::atomic::{AtomicU16, Ordering};
use crate::{Rooms};
use crate::client_message::{ClientResponse, ClientRequest};
use crate::client::Client;
use uuid::Uuid;
use crate::room_message::RoomMessage;
use tokio::sync::oneshot::error::TryRecvError;

pub async fn user_connected(ws: WebSocket, rooms: Rooms){
    //Bind mpsc channel to websocket
    let (web_tx, web_rx) = mpsc::unbounded_channel();
    let (user_ws_tx, mut user_ws_rx) = ws.split();
    tokio::task::spawn(
        web_rx.forward(user_ws_tx)
            .map(|result| {
                if let Err(e) = result {
                    eprintln!("websocket send error: {}", e);
                }
            })
    );

    //Heartbeat
    let ping_counter = Arc::new(AtomicU16::new(0));
    let pong_counter = Arc::new(AtomicU16::new(0));

    let tx = web_tx.clone();
    let ping_cnt = ping_counter.clone();
    let pong_cnt = pong_counter.clone();
    //hb_tx goes out of scope, ending the task
    let (hb_tx, mut hb_rx) = tokio::sync::oneshot::channel::<()>();
    tokio::spawn(async move {
        let mut heartbeat_interval = tokio::time::interval(tokio::time::Duration::from_secs(5));
        loop {
            heartbeat_interval.tick().await;
            tx.send(Ok(warp::ws::Message::ping(Vec::new())));
            ping_cnt.fetch_add(1, Ordering::Relaxed);
            let pings = ping_cnt.load(Ordering::Relaxed);
            let pongs = pong_cnt.load(Ordering::Relaxed);
            if pings > pongs + 1 + 5 {
                //this user is probably not alive
                println!("One sided disconnect");
                //TODO make ws actually disconnect...
                break;
            }
            match hb_rx.try_recv() {
                // The channel will never receive a value.
                Ok(_) | Err(TryRecvError::Closed) => {
                    break;
                }
                Err(TryRecvError::Empty) => {}
            }
        }
    });

    //This Client properties
    let my_id = Uuid::new_v4();
    let mut my_room:Option<(String, mpsc::UnboundedSender<RoomMessage>)> = None;
    let name = generate_name();
    //Insert client to global map of clients
    let my_client = Arc::new(Client{
        name,
        id: my_id.clone(),
        sender: web_tx.clone()
    });

    {
        let mut rms = rooms.write().await;
        rms.register_broadcast_rooms(my_id, my_client.clone()).await;
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

        if msg.is_pong() {
            pong_counter.fetch_add(1, Ordering::Relaxed);
        }

        if let Ok(str) = msg.to_str() {
            let res:serde_json::Result<ClientRequest> = serde_json::from_str(str);
            if let Ok(cm) = res {
                match cm {
                    ClientRequest::ListRooms => {
                        let public_rooms = rooms.read().await.get_public_rooms().await;
                        my_client.try_send(ClientResponse::RoomList(public_rooms));
                    }
                    ClientRequest::CreateRoom{allow_edits, is_public, init_game_state } => {
                        if my_room.is_none() {
                            //Get a new ID
                            let room_id = {
                                rooms.read().await.get_new_id().await
                            };
                            println!("New room id: {}", room_id);
                            let mut success = false;
                            match rooms.write().await.new_room(room_id.clone(), is_public, allow_edits, init_game_state).await {
                                Ok(_tx) => {
                                    success = true;
                                    my_client.try_send(ClientResponse::RoomCreateSuccess(room_id.clone()));
                                }
                                Err(_) => {
                                    my_client.try_send(ClientResponse::CannotOverwriteRoom);
                                }
                            }
                            if success {
                                match rooms.read().await.add_client_to_room(&room_id, my_client.clone()).await {
                                    Ok(tx) =>{
                                        my_room = Some((room_id, tx));
                                    }
                                    Err(_) => {
                                        my_client.try_send(ClientResponse::NoRoomFound) ;
                                    }
                                }
                            }
                            if my_room.is_some(){
                                //Don't listen to room changes anymore
                                let mut rms = rooms.write().await;
                                rms.unregister_broadcast_rooms(&my_id).await;
                            }
                        }
                    }
                    ClientRequest::JoinRoom(room_id) => {
                        println!("Join room requested!");
                        if my_room.is_none(){
                            match rooms.read().await.add_client_to_room(&room_id, my_client.clone()).await {
                                Ok(tx) =>{
                                    my_room = Some((room_id, tx));
                                }
                                Err(_) => {
                                    my_client.try_send(ClientResponse::NoRoomFound) ;
                                }
                            }
                        }
                        if my_room.is_some(){
                            //Don't listen to room changes anymore
                            let mut rms = rooms.write().await;
                            rms.unregister_broadcast_rooms(&my_id).await;
                        }
                    }
                    ClientRequest::LeaveRoom => {
                        if let Some((room_id, _tx)) = my_room {
                            {
                                rooms.read().await.remove_client_from_room(&room_id, &my_id).await;
                                my_room = None;
                            }
                            //Listen to room changes again once out of room
                            let mut rms = rooms.write().await;
                            rms.register_broadcast_rooms(my_id.clone(), my_client.clone()).await;
                        }
                        my_client.try_send(ClientResponse::RemovedFromRoom);
                    }
                    // Forward any other valid ClientRequest to the room the user is in
                    _ => {
                        if let Some((_room_id, room_tx)) = my_room.clone() {
                            if let Err(_) = room_tx.send(RoomMessage::External(my_id.clone(), cm)){
                                eprintln!("Client error sending to room");
                            }
                        }
                    }
                }
            }else{
                // If we ever get here, the user sent some garbage. Disconnect them.
                break;
            }
        }
    }

    //If we get here that means the user disconnected
    eprintln!("good bye user: {}", my_id);
    if let Some((room_id, _tx)) = my_room {
        rooms.read().await.remove_client_from_room(&room_id, &my_id).await;
    }

    let mut rms = rooms.write().await;
    rms.unregister_broadcast_rooms(&my_id).await;
}

fn generate_name() -> String {
    let mut generator = adjective_adjective_animal::Generator::default();
    if let Some(name) = generator.next() {
        name
    }else{
        "Anon".to_string()
    }
}
