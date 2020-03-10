import {BoardLocation} from "./BoardLocation";
import {MovementPattern} from "./MovementPattern";
import {GameState} from "./GameState";
import {Movement} from "./Movement";
import {PieceType} from "./PieceType";

export abstract class Piece {
    //The movement patterns that this piece uses
    movementPatterns:MovementPattern[];
    owner:number; //who owns this piece? 0 for white, 1 for black 2......
    location:BoardLocation;
    pieceType:PieceType;
    constructor(owner:number,location:BoardLocation) {
        this.pieceType = PieceType.Custom;
        this.owner = owner;
        this.movementPatterns = [];
        this.location = location;
    }
    getPossibleMoves(gameState:GameState):Set<Movement>{
        if (this.location) {
            let returnMoves:Movement[] = [];
            for (let pattern of this.movementPatterns) {
                returnMoves.push(...Array.from(pattern.getPossibleMoves(this.location, gameState)));
            }
            return new Set<Movement>(returnMoves);
        }else{
            return new Set<Movement>();
        }
    }
    toAscii(){
        return 'w';
    }
}