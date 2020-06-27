use tokio::sync::mpsc;
use crate::room_message::RoomMessage;
use crate::client::Client;
use crate::client_message::{ClientRequest, ClientResponse};
use protochess_common::{Piece, Turn, serialize_game_state, validate_gamestate_request};
use std::sync::{ Arc };
use uuid::Uuid;
use lazy_static::lazy_static;


lazy_static! {
    static ref MOVEGEN:protochess_engine_rs::MoveGenerator = {
        protochess_engine_rs::MoveGenerator::new()
    };
}


pub struct Room {
    pub editable: bool,
    //clients[0] is the leader
    pub(crate) game: protochess_engine_rs::Game,
    to_move_in_check: bool,
    winner: Option<String>,
    clients: Vec<Arc<Client>>,
    last_turn: Option<Turn>,
    rx: mpsc::UnboundedReceiver<RoomMessage>,
}

impl Room {
    pub fn new(rx: mpsc::UnboundedReceiver<RoomMessage>) -> Room {
        Room{
            game: protochess_engine_rs::Game::default(),
            to_move_in_check: false,
            editable: true,
            winner: None,
            clients: Vec::new(),
            last_turn: None,
            rx,
        }
    }

    //Technically only ever runs in one process (that's why self.clients is not behind a Mutex)
    //Need async here to call recv.await
    pub async fn run(&mut self){
        while let Some(message) = self.rx.recv().await {
            match message {
                RoomMessage::AddClient(client) => {
                    client.try_send(self.serialize_game());
                    self.clients.push(client);
                    self.broadcast_player_list();
                }
                RoomMessage::RemoveClient(id) => {
                    if let Some(index) = self.clients.iter().position(|x| x.id == id){
                        self.clients.remove(index);
                        //Broadcast the new player list
                        self.broadcast_player_list();
                    }else{
                        eprintln!("no user found at id");
                    }
                }
                RoomMessage::External(requester_id, client_request) => {
                    if let Some(player_num) = self.clients.iter().position(|x| x.id == requester_id){
                        let requester_client = &self.clients[player_num];
                        match client_request {
                            ClientRequest::DisableEdits => {
                                if player_num == 0 {
                                    self.editable = false;
                                    self.broadcast_game_update();
                                }
                            }
                            ClientRequest::ChatMessage(m) => {
                                //Send message to other users in the room
                                self.broadcast_except(ClientResponse::ChatMessage {
                                    from: format!("{}", &requester_client.name),
                                    content: format!("{}", m)
                                }, requester_id);
                            }
                            ClientRequest::TakeTurn(turn) => {
                                let from = turn.from;
                                let to = turn.to;
                                //Check if it's this player's turn
                                if player_num as u8 == self.game.get_whos_turn() {
                                    let (x1, y1) = from;
                                    let (x2, y2) = to;
                                    let move_gen:&protochess_engine_rs::MoveGenerator = &MOVEGEN;
                                    if self.game.make_move(move_gen, x1, y1, x2, y2){
                                        // TODO add promotion
                                        self.last_turn = Some(Turn {
                                            promote_to: None,
                                            from,
                                            to
                                        });

                                        //Calculate if the position is in check after making this move
                                        self.to_move_in_check = move_gen.in_check(&mut self.game.current_position);
                                        if self.to_move_in_check {
                                            if move_gen.count_legal_moves(&mut self.game.current_position) == 0 {
                                                //We have a winner!
                                                self.winner = Some(requester_client.name.clone());
                                            }
                                        }
                                        //See if we have any more moves
                                        self.broadcast_game_update();

                                    }
                                }

                            }
                            ClientRequest::GameState => {
                                requester_client.try_send(self.serialize_game());
                            }
                            ClientRequest::EditGameState(request_game_state) => {
                                //TODO 2+ players ?
                                if self.editable && player_num < 2 {
                                    if let Some((movements, valid_squares, valid_pieces)) =
                                    validate_gamestate_request(request_game_state.tiles,
                                                                     request_game_state.pieces,
                                                                     request_game_state.movement_patterns){
                                        self.game.set_state(movements,
                                                            valid_squares,
                                                            valid_pieces);
                                        self.broadcast_game_update();
                                    }
                                }
                            }
                            ClientRequest::SwitchLeader(new_leader) => {
                                if player_num == 0 && (new_leader as usize) < self.clients.len() {
                                    self.clients.swap(0, new_leader as usize);
                                }
                            }
                            ClientRequest::ListPlayers => {
                                requester_client.try_send(ClientResponse::PlayerList {
                                    player_num: player_num as u8,
                                    you: format!("{}", requester_client.name),
                                    names: self.clients.iter().map(|x| x.name.clone()).collect()
                                })
                            }
                            ClientRequest::MovesFrom(x, y) => {
                                if player_num as u8 == self.game.get_whos_turn() {
                                    let mut possible_moves = Vec::new();
                                    let move_gen:&protochess_engine_rs::MoveGenerator = &MOVEGEN;
                                    for (from, to) in  move_gen.get_legal_moves_as_tuples(&mut self.game.current_position){
                                        if from == (x, y){
                                            possible_moves.push(to);
                                        }
                                    }
                                    requester_client.try_send(ClientResponse::MovesFrom {
                                        from: (x, y),
                                        to: possible_moves
                                    });
                                }
                            }
                            _ => {}
                        }
                    }else{
                        eprintln!("External room request from unknown client")
                    }
                }
            }

            //Leave if room is empty
            if self.clients.len() == 0 {
                break;
            }
        }
    }

    fn broadcast_game_update(&mut self){
        self.broadcast(self.serialize_game());
    }

    fn serialize_game(&self) -> ClientResponse {
        let to_move = self.game.get_whos_turn();
        let to_move_in_check = self.to_move_in_check;
        let in_check_kings = if to_move_in_check {
            Some(
                self.game.current_position.pieces_as_tuples()
                    .into_iter()
                    .filter(|(owner, _x, _y, piece_type)|{
                        *owner == to_move && *piece_type == 'k'
                    })
                    .map(|(owner, x, y, piece_type)|{
                        Piece {
                            owner,
                            x,
                            y,
                            piece_type
                        }
                    }).collect()
            )
        } else { None };

        ClientResponse::GameInfo {
            editable: self.editable,
            winner: self.winner.clone(),
            last_turn: (&self.last_turn).to_owned(),
            to_move_in_check,
            in_check_kings,
            state: serialize_game_state(&self.game.current_position)
        }
    }

    /// Sends a message to everyone except Uuid
    fn broadcast_except(&self, cr: ClientResponse, except_id: Uuid) {
        for client in &self.clients {
            if client.id != except_id {
                client.try_send(cr.clone());
            }
        }
    }

    /// Sends a message to everyone
    fn broadcast(&self, cr: ClientResponse) {
        for client in &self.clients {
            client.try_send(cr.clone());
        }
    }

    fn broadcast_player_list(&self){
        for (i, client) in self.clients.iter().enumerate() {
            client.try_send(ClientResponse::PlayerList {
                player_num: i as u8,
                you: format!("{}", client.name),
                names: self.clients.iter().map(|x| x.name.clone()).collect()
            });
        }
    }
}
