import {MapSchema, Schema, type} from "@colyseus/schema";
import {Player} from './Player';
import {GameState} from './chess/GameState'

const anonymus = require("anonymus");

export class State extends Schema {
    @type({map: Player})
    players = new MapSchema<Player>();

    @type(GameState)
    gameState = new GameState();


    createPlayer(id: string, isLeader: boolean, name: string) {
        let newPlayer = new Player();
        newPlayer.isLeader = isLeader;
        if (name == "") newPlayer.name = anonymus.create()[0];
        else newPlayer.name = name;

        newPlayer.id = id;

        this.players[id] = newPlayer;
    }

    removePlayer(id: string) {
        delete this.players[id];
    }
}
