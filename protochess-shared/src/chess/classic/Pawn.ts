import {Piece} from "../Piece";
import {MovementPattern} from "../MovementPattern";
import {GameState} from "../GameState";
import {BoardLocation} from "../BoardLocation";
import {Movement} from "../Movement";
import {PieceType} from "../PieceType";
import {CapturePattern} from "../CapturePattern";

export class Pawn extends Piece{
    startLoc: BoardLocation;
    constructor(owner:number,location:BoardLocation) {
        super(owner,location);
        this.pieceType = PieceType.Pawn;
        this.startLoc = location;

        if (owner == 0) {
            this.movementPatterns.push(new MovementPattern(owner, 0, 1, 2, true));
            this.capturePatterns.push(new CapturePattern(owner,-1,1,1,true));
            this.capturePatterns.push(new CapturePattern(owner,1,1,1,true));

        }else{
            this.movementPatterns.push(new MovementPattern(owner, 0, -1, 2, true));
            this.capturePatterns.push(new CapturePattern(owner,-1,-1,1,true));
            this.capturePatterns.push(new CapturePattern(owner,1,-1,1,true));
        }

    }

    getPossibleMoves(gameState: GameState): Set<Movement> {
        if (!(this.startLoc.x  == this.location.x && this.startLoc.y == this.location.y)){
            //Moved already; can't move by 2
            this.movementPatterns[0].stepMax = 1;
        }else{
            this.movementPatterns[0].stepMax = 2;
        }

        return super.getPossibleMoves(gameState);
    }

    toAscii(): string {
        return this.owner == 0 ? 'P' : 'p';
    }

}