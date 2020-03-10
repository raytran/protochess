import {ArraySchema,MapSchema, Schema, type} from "@colyseus/schema";
const anonymus = require("anonymus");
export class PieceType extends Schema {
    @type("string")
    asciiRep = "";

    @type("string")
    owner = "black";

    @type([ "string" ])
    movements = new ArraySchema<string>();
}

export class TileType extends Schema {
    @type("string")
    asciiRep = "b";

    constructor(type:string) {
        super();
    }

}
export class Board extends Schema {
    @type("number")
    width = 8;

    @type("number")
    height = 8;

    @type("string")
    asciiRep = "";

    @type([TileType])
    tileTypes = new MapSchema<TileType>(); //Maps ascii rep to tile type

    @type([PieceType])
    pieceTypes = new MapSchema<PieceType>(); //Maps ascii rep to piece type
}
export class Player extends Schema {
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

    @type(Board)
    board = null;

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
}
