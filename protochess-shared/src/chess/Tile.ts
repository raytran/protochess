//Tile class; might have tiles with special properties later
import {Schema, type} from "@colyseus/schema";

export class Tile extends Schema {
    @type("number")
    x: number;
    @type("number")
    y: number;
    @type('string')
    tileTypeStr: string;

    constructor(x: number, y: number, type: string) {
        super();
        this.x = x;
        this.y = y;
        this.tileTypeStr = type;
    }
}
