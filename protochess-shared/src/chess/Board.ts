//Holds Tiles
import {Tile} from "./Tile";
import {ArraySchema, Schema, type} from "@colyseus/schema";

//Represents just the tiles NOT pieces
export class Board extends Schema {
    @type([Tile])
    tiles: ArraySchema<Tile>;
    @type('number')
    width: number;
    @type('number')
    height: number;

    constructor() {
        super();
        this.width = 8;
        this.height = 8;
        this.tiles = new ArraySchema<Tile>();
    }
}