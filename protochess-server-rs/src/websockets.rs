use tokio::sync::{mpsc};
use futures::{FutureExt, StreamExt};
use warp::ws::WebSocket;
use warp::Filter;
use std::sync::{ Arc };
use crate::{Rooms};
use crate::client_message::{ClientResponse, ClientRequest};
use crate::client::Client;
use uuid::Uuid;
use crate::room_message::RoomMessage;

pub async fn user_connected(ws: WebSocket, rooms: Rooms){
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

    //This Client properties
    let my_id = Uuid::new_v4();
    let mut my_room:Option<(String, mpsc::UnboundedSender<RoomMessage>)> = None;
    let mut generator = adjective_adjective_animal::Generator::default();
    let name = generator.next().unwrap();
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

        if let Ok(str) = msg.to_str(){
            let res:serde_json::Result<ClientRequest> = serde_json::from_str(str);
            if let Ok(cm) = res {
                match cm {
                    ClientRequest::ListRooms => {
                        let public_rooms = rooms.read().await.get_public_room_ids().await;
                        my_client.try_send(ClientResponse::RoomList(public_rooms));
                    }
                    ClientRequest::CreateRoom{room_id, is_public} => {
                        println!("Create room requested!");
                        if my_room.is_none() {
                            if let Err(_) = rooms.write().await.new_room(room_id.clone(), is_public).await {
                                my_client.try_send(ClientResponse::CannotOverwriteRoom);
                            };

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
                        println!("Leave room requested!");
                        if let Some((room_id, tx)) = my_room {
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
                        if let Some((room_id, room_tx)) = my_room.clone() {
                            if let Err(_) = room_tx.send(RoomMessage::External(my_id.clone(), cm)){
                                eprintln!("Client error sending to room");
                            }
                        }
                    }
                }
            }
        }
    }

    //If we get here that means the user disconnected
    eprintln!("good bye user: {}", my_id);
    if let Some((room_id, tx)) = my_room {
       rooms.read().await.remove_client_from_room(&room_id, &my_id).await;
    }

    let mut rms = rooms.write().await;
    rms.unregister_broadcast_rooms(&my_id).await;
}

