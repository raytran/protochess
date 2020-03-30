import {Schema, type} from "@colyseus/schema";

export class Piece extends Schema {
    @type("number")
    owner: number; //who owns this piece? 0 for white, 1 for black 2......
    @type("number")
    x: number;
    @type("number")
    y: number;
    @type("string")
    pieceTypeStr: string;
    @type("string")
    id: string;

    constructor(owner: number, x: number, y: number, pieceTypeStr: string, id: string) {
        super();
        this.owner = owner;
        this.x = x;
        this.y = y;
        this.id = id;
        this.pieceTypeStr = pieceTypeStr;
    }
}