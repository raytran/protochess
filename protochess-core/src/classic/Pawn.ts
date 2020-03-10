import {Piece} from "../Piece";
import {MovementPattern} from "../MovementPattern";
import {GameState} from "../GameState";
import {BoardLocation} from "../BoardLocation";
import {Movement} from "../Movement";
import {PieceType} from "../PieceType";

export class Pawn extends Piece{
    startLoc: BoardLocation;
    constructor(owner:number,location:BoardLocation) {
        super(owner,location);
        this.pieceType = PieceType.Pawn;
        this.startLoc = location;
        if (owner == 0) {
            this.movementPatterns.push(new MovementPattern(owner, 0, 1, 2, true));
        }else{
            this.movementPatterns.push(new MovementPattern(owner, 0, -1, 2, true));
        }
    }

    getPossibleMoves(gameState: GameState): Set<Movement> {
        if (this.startLoc != this.location){
            //Moved already; can't move by 2
            this.movementPatterns[0].stepMax = 1;
        }

        return super.getPossibleMoves(gameState);
    }

    toAscii(): string {
        return this.owner == 0 ? 'P' : 'p';
    }

}