import {MapSchema, Schema, type} from "@colyseus/schema";
const anonymus = require("anonymus");
export class Player extends Schema {
    @type("number")
    x = Math.floor(Math.random() * 400);

    @type("number")
    y = Math.floor(Math.random() * 400);

    @type("boolean")
    isLeader= false;

    @type("string")
    id = "";

    @type("string")
    name = "Anon";
}

export class State extends Schema {
    @type({map: Player})
    players = new MapSchema<Player>();

    createPlayer(id: string,isLeader:boolean,name:string){
        let newPlayer = new Player();
        newPlayer.isLeader = isLeader;
        if (name=="") newPlayer.name = anonymus.create()[0];
        else newPlayer.name = name;

        newPlayer.id = id;

        this.players[id] = newPlayer;
    }

    removePlayer(id: string) {
        delete this.players[id];
    }

    movePlayer(id: string, movement: any) {
        if (movement.x) {
            this.players[id].x += movement.x * 10;

        } else if (movement.y) {
            this.players[id].y += movement.y * 10;
        }
    }
}
