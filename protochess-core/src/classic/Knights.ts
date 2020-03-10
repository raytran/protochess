import {Piece} from "../Piece";
import {MovementPattern} from "../MovementPattern";
import {BoardLocation} from "../BoardLocation";
import {PieceType} from "../PieceType";

export class Knight extends Piece{
    constructor(owner:number,location:BoardLocation) {
        super(owner,location);
        this.pieceType = PieceType.Knight;
        this.movementPatterns.push(new MovementPattern(owner,2,1,1,true));
        this.movementPatterns.push(new MovementPattern(owner,-2,1,1,true));
        this.movementPatterns.push(new MovementPattern(owner,-2,-1,1,true));
        this.movementPatterns.push(new MovementPattern(owner,2,-1,1,true));
        this.movementPatterns.push(new MovementPattern(owner,1,2,1,true));
        this.movementPatterns.push(new MovementPattern(owner,-1,2,1,true));
        this.movementPatterns.push(new MovementPattern(owner,1,-2,1,true));
        this.movementPatterns.push(new MovementPattern(owner,-1,-2,1,true));
    }
    toAscii(): string {
        return this.owner == 0 ? 'N' : 'n';
    }

}