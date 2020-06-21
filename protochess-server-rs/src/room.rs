use tokio::sync::mpsc;
use crate::room_message::RoomMessage;
use crate::client::Client;
use crate::client_message::{ClientRequest, ClientResponse, Piece, Tile, Turn, MovementPattern, Slides};
use std::sync::{ Arc };
use std::collections::HashMap;
use uuid::Uuid;
use lazy_static::lazy_static;
use protochess_engine_rs::MovementPatternExternal;


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
                                if self.editable {
                                    if let Some((movements, valid_squares, valid_pieces)) =
                                    Room::validate_gamestate_request(request_game_state.tiles,
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
        let width = self.game.get_width();
        let height = self.game.get_height();
        let pieces = self.game.current_position.pieces_as_tuples()
            .into_iter()
            .map(|(owner, x, y, piece_type)| {
                Piece{
                    owner,
                    x,
                    y,
                    piece_type
                }
            })
            .collect();
        let tiles = self.game.current_position.tiles_as_tuples()
            .into_iter()
            .map(|(x, y, tile_type)|{
                Tile{
                    x,
                    y,
                    tile_type
                }
            })
            .collect();
        let to_move = self.game.get_whos_turn();
        let to_move_in_check = self.to_move_in_check;
        let in_check_kings = if to_move_in_check {
            Some(
                self.game.current_position.pieces_as_tuples()
                    .into_iter()
                    .filter(|(owner, x, y, piece_type)|{
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

        let map = self.game.current_position.get_char_movementpattern_map();

        let movement_patterns = {
            let mut temp = HashMap::new();
            for (k, v) in self.game.current_position.get_char_movementpattern_map() {
                temp.insert(k, MovementPattern{
                    attack_slides: Slides {
                        north: v.attack_north,
                        east: v.attack_east,
                        south: v.attack_south,
                        west: v.attack_west,
                        northeast: v.attack_northeast,
                        northwest: v.attack_northwest,
                        southeast: v.attack_southeast,
                        southwest: v.attack_southwest
                    },
                    translate_slides: Slides {
                        north: v.translate_north,
                        east: v.translate_east,
                        south: v.translate_south,
                        west: v.translate_west,
                        northeast: v.translate_northeast,
                        northwest: v.translate_northwest,
                        southeast: v.translate_southeast,
                        southwest: v.translate_southwest
                    },
                    attack_jumps: v.attack_jump_deltas,
                    translate_jumps: v.translate_jump_deltas,
                    attack_slide_deltas: v.attack_sliding_deltas,
                    translate_slide_deltas:v.translate_sliding_deltas
                });
            }
            temp
        };




        ClientResponse::GameState {
            editable: self.editable,
            width,
            height,
            winner: self.winner.clone(),
            to_move,
            to_move_in_check,
            in_check_kings,
            last_turn: (&self.last_turn).to_owned(),
            tiles,
            pieces,
            movement_patterns
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

    pub(crate) fn validate_gamestate_request(tiles:Vec<Tile>, pieces:Vec<Piece>, movement_patterns:HashMap<char, MovementPattern>)
                                             -> Option<(HashMap<char,MovementPatternExternal>, Vec<(u8, u8)>, Vec<(u8, u8, u8, char)>)>{
        let mut w_has_king = false;
        let mut b_has_king = false;
        for pce in &pieces {
            if pce.piece_type == 'k' {
                if pce.owner == 0 {
                    w_has_king = true;
                }else {
                    b_has_king = true;
                }
            }
        }

        if w_has_king && b_has_king {
            let valid_squares = tiles
                .into_iter()
                .filter(|sq| sq.x < 16 && sq.y < 16 && (sq.tile_type == 'w' || sq.tile_type == 'b'))
                .map(|sq| (sq.x, sq.y))
                .collect();

            let mut movements = HashMap::new();
            for (key, val) in movement_patterns {
                let piece_type_char = key.to_ascii_lowercase();
                if piece_type_char.is_ascii_alphabetic(){
                    movements.insert(piece_type_char, protochess_engine_rs::MovementPatternExternal{
                        promotion_squares: None,
                        promo_vals: None,
                        attack_sliding_deltas: val.attack_slide_deltas,
                        attack_jump_deltas: val.attack_jumps,
                        attack_north: val.attack_slides.north,
                        attack_south: val.attack_slides.south,
                        attack_east: val.attack_slides.east,
                        attack_west: val.attack_slides.west,
                        attack_northeast: val.attack_slides.northeast,
                        attack_northwest: val.attack_slides.northwest,
                        attack_southeast: val.attack_slides.southeast,
                        attack_southwest: val.attack_slides.southwest,
                        translate_jump_deltas: val.translate_jumps,
                        translate_sliding_deltas: val.translate_slide_deltas,
                        translate_north: val.translate_slides.north,
                        translate_south: val.translate_slides.south,
                        translate_east: val.translate_slides.east,
                        translate_west: val.translate_slides.west,
                        translate_northeast: val.translate_slides.northeast,
                        translate_northwest: val.translate_slides.northwest,
                        translate_southeast: val.translate_slides.southeast,
                        translate_southwest: val.translate_slides.southwest
                    });
                }
            }

            let pieces = pieces.into_iter().map(|pce| (pce.owner, pce.x, pce.y, pce.piece_type)).collect();

            return Some((movements, valid_squares, pieces));
        }
        None
    }
}
