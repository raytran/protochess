import {derived, writable} from "svelte/store";

const URI = "ws://localhost:3030/chess/bruh";
const socket = new WebSocket(URI);


let rawData = JSON.parse("{\"GameState\":{\"width\":8,\"height\":8,\"to_move\":0,\"tiles\":[{\"x\":0,\"y\":0,\"tile_type\":\"b\"},{\"x\":0,\"y\":1,\"tile_type\":\"w\"},{\"x\":0,\"y\":2,\"tile_type\":\"b\"},{\"x\":0,\"y\":3,\"tile_type\":\"w\"},{\"x\":0,\"y\":4,\"tile_type\":\"b\"},{\"x\":0,\"y\":5,\"tile_type\":\"w\"},{\"x\":0,\"y\":6,\"tile_type\":\"b\"},{\"x\":0,\"y\":7,\"tile_type\":\"w\"},{\"x\":1,\"y\":0,\"tile_type\":\"w\"},{\"x\":1,\"y\":1,\"tile_type\":\"b\"},{\"x\":1,\"y\":2,\"tile_type\":\"w\"},{\"x\":1,\"y\":3,\"tile_type\":\"b\"},{\"x\":1,\"y\":4,\"tile_type\":\"w\"},{\"x\":1,\"y\":5,\"tile_type\":\"b\"},{\"x\":1,\"y\":6,\"tile_type\":\"w\"},{\"x\":1,\"y\":7,\"tile_type\":\"b\"},{\"x\":2,\"y\":0,\"tile_type\":\"b\"},{\"x\":2,\"y\":1,\"tile_type\":\"w\"},{\"x\":2,\"y\":2,\"tile_type\":\"b\"},{\"x\":2,\"y\":3,\"tile_type\":\"w\"},{\"x\":2,\"y\":4,\"tile_type\":\"b\"},{\"x\":2,\"y\":5,\"tile_type\":\"w\"},{\"x\":2,\"y\":6,\"tile_type\":\"b\"},{\"x\":2,\"y\":7,\"tile_type\":\"w\"},{\"x\":3,\"y\":0,\"tile_type\":\"w\"},{\"x\":3,\"y\":1,\"tile_type\":\"b\"},{\"x\":3,\"y\":2,\"tile_type\":\"w\"},{\"x\":3,\"y\":3,\"tile_type\":\"b\"},{\"x\":3,\"y\":4,\"tile_type\":\"w\"},{\"x\":3,\"y\":5,\"tile_type\":\"b\"},{\"x\":3,\"y\":6,\"tile_type\":\"w\"},{\"x\":3,\"y\":7,\"tile_type\":\"b\"},{\"x\":4,\"y\":0,\"tile_type\":\"b\"},{\"x\":4,\"y\":1,\"tile_type\":\"w\"},{\"x\":4,\"y\":2,\"tile_type\":\"b\"},{\"x\":4,\"y\":3,\"tile_type\":\"w\"},{\"x\":4,\"y\":4,\"tile_type\":\"b\"},{\"x\":4,\"y\":5,\"tile_type\":\"w\"},{\"x\":4,\"y\":6,\"tile_type\":\"b\"},{\"x\":4,\"y\":7,\"tile_type\":\"w\"},{\"x\":5,\"y\":0,\"tile_type\":\"w\"},{\"x\":5,\"y\":1,\"tile_type\":\"b\"},{\"x\":5,\"y\":2,\"tile_type\":\"w\"},{\"x\":5,\"y\":3,\"tile_type\":\"b\"},{\"x\":5,\"y\":4,\"tile_type\":\"w\"},{\"x\":5,\"y\":5,\"tile_type\":\"b\"},{\"x\":5,\"y\":6,\"tile_type\":\"w\"},{\"x\":5,\"y\":7,\"tile_type\":\"b\"},{\"x\":6,\"y\":0,\"tile_type\":\"b\"},{\"x\":6,\"y\":1,\"tile_type\":\"w\"},{\"x\":6,\"y\":2,\"tile_type\":\"b\"},{\"x\":6,\"y\":3,\"tile_type\":\"w\"},{\"x\":6,\"y\":4,\"tile_type\":\"b\"},{\"x\":6,\"y\":5,\"tile_type\":\"w\"},{\"x\":6,\"y\":6,\"tile_type\":\"b\"},{\"x\":6,\"y\":7,\"tile_type\":\"w\"},{\"x\":7,\"y\":0,\"tile_type\":\"w\"},{\"x\":7,\"y\":1,\"tile_type\":\"b\"},{\"x\":7,\"y\":2,\"tile_type\":\"w\"},{\"x\":7,\"y\":3,\"tile_type\":\"b\"},{\"x\":7,\"y\":4,\"tile_type\":\"w\"},{\"x\":7,\"y\":5,\"tile_type\":\"b\"},{\"x\":7,\"y\":6,\"tile_type\":\"w\"},{\"x\":7,\"y\":7,\"tile_type\":\"b\"}],\"pieces\":[{\"owner\":0,\"x\":4,\"y\":0,\"piece_type\":\"k\"},{\"owner\":0,\"x\":3,\"y\":0,\"piece_type\":\"q\"},{\"owner\":0,\"x\":2,\"y\":0,\"piece_type\":\"b\"},{\"owner\":0,\"x\":5,\"y\":0,\"piece_type\":\"b\"},{\"owner\":0,\"x\":1,\"y\":0,\"piece_type\":\"n\"},{\"owner\":0,\"x\":6,\"y\":0,\"piece_type\":\"n\"},{\"owner\":0,\"x\":0,\"y\":0,\"piece_type\":\"r\"},{\"owner\":0,\"x\":7,\"y\":0,\"piece_type\":\"r\"},{\"owner\":0,\"x\":0,\"y\":1,\"piece_type\":\"p\"},{\"owner\":0,\"x\":1,\"y\":1,\"piece_type\":\"p\"},{\"owner\":0,\"x\":2,\"y\":1,\"piece_type\":\"p\"},{\"owner\":0,\"x\":3,\"y\":1,\"piece_type\":\"p\"},{\"owner\":0,\"x\":4,\"y\":1,\"piece_type\":\"p\"},{\"owner\":0,\"x\":5,\"y\":1,\"piece_type\":\"p\"},{\"owner\":0,\"x\":6,\"y\":1,\"piece_type\":\"p\"},{\"owner\":0,\"x\":7,\"y\":1,\"piece_type\":\"p\"},{\"owner\":1,\"x\":4,\"y\":7,\"piece_type\":\"k\"},{\"owner\":1,\"x\":3,\"y\":7,\"piece_type\":\"q\"},{\"owner\":1,\"x\":2,\"y\":7,\"piece_type\":\"b\"},{\"owner\":1,\"x\":5,\"y\":7,\"piece_type\":\"b\"},{\"owner\":1,\"x\":1,\"y\":7,\"piece_type\":\"n\"},{\"owner\":1,\"x\":6,\"y\":7,\"piece_type\":\"n\"},{\"owner\":1,\"x\":0,\"y\":7,\"piece_type\":\"r\"},{\"owner\":1,\"x\":7,\"y\":7,\"piece_type\":\"r\"},{\"owner\":1,\"x\":0,\"y\":6,\"piece_type\":\"p\"},{\"owner\":1,\"x\":1,\"y\":6,\"piece_type\":\"p\"},{\"owner\":1,\"x\":2,\"y\":6,\"piece_type\":\"p\"},{\"owner\":1,\"x\":3,\"y\":6,\"piece_type\":\"p\"},{\"owner\":1,\"x\":4,\"y\":6,\"piece_type\":\"p\"},{\"owner\":1,\"x\":5,\"y\":6,\"piece_type\":\"p\"},{\"owner\":1,\"x\":6,\"y\":6,\"piece_type\":\"p\"},{\"owner\":1,\"x\":7,\"y\":6,\"piece_type\":\"p\"}]}}");
const _GameState = writable(rawData['GameState']);
const _ChatMessage = writable(null);
const _PlayersList = writable({player_num: 0, you: "You", names:[]});

socket.onopen = function(){
    console.log("socket opened!");
}

socket.onmessage = function(msg){
    try {
        let apiMsg = JSON.parse(msg.data);
        if (apiMsg.type === 'GameState') {
            _GameState.set(apiMsg.content);
        }else if (apiMsg.type === 'ChatMessage') {
            _ChatMessage.set(apiMsg.content);
        }else if (apiMsg.type === 'PlayerList'){
            console.log(apiMsg.content);
            _PlayersList.set(apiMsg.content);
        }
    }catch (e) {
        console.log("received bad JSON from socket");
    }
}

export function sendChatMessage(msg) {
    let apiMsg = {"content":msg,"type":"ChatMessage"};
    socket.send(JSON.stringify(apiMsg));
}

// [x, y]
export function sendTakeTurn(from, to){
    let apiMsg = {"content":{"from":from,"to":to},"type":"TakeTurn"};
    let json = JSON.stringify(apiMsg);
    console.log(json);
    socket.send(json);
}

export const GameState = derived(_GameState, x => x);
export const ChatMessage = derived(_ChatMessage, x => x);
export const PlayersList = derived(_PlayersList, x => x);


