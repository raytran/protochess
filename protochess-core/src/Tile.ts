//Tile class; might have tiles with special properties later
import {TileType} from "./TileType";
import {BoardLocation} from "./BoardLocation";

export class Tile{
    type:TileType;
    location:BoardLocation;

    constructor(location:BoardLocation,type:TileType) {
        this.location = location;
        this.type = type;
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