//Tile class; might have tiles with special properties later
import {TileType} from "./TileType";
import {BoardLocation} from "./BoardLocation";
import {Schema, type} from "@colyseus/schema";

export class Tile extends Schema{
    @type(BoardLocation)
    location:BoardLocation;
    @type('string')
    tileTypeStr:string;

    type:TileType;
    constructor(location:BoardLocation,type:TileType) {
        super();
        this.location = location;
        this.type = type;
        this.tileTypeStr = this.toAscii();
    }

    toAscii(){
        switch(this.type){
            case TileType.Black:
                return 'b';
            case TileType.White:
                return 'w';
        }
    }
}