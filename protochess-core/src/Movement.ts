import {BoardLocation} from "./BoardLocation";
import {Piece} from "./Piece";
//Represents a move from one location to another
export class Movement{
    capturedPiece:Piece | null = null; //piece that this move captures
    start:BoardLocation;
    end:BoardLocation;
    constructor(start:BoardLocation,end:BoardLocation) {
        if (start.x == end.x && start.y == end.y){
            throw new Error("Start and ends cannot be equal");
        }
        this.start = start;
        this.end = end;
    }
}