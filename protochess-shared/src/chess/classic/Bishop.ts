import {Piece} from "../Piece";
import {MovementPattern} from "../MovementPattern";
import {BoardLocation} from "../BoardLocation";
import {PieceType} from "../PieceType";
import {CapturePattern} from "../CapturePattern";

export class Bishop extends Piece{
    constructor(owner:number,location:BoardLocation) {
        super(owner,location);
        this.pieceType = PieceType.Bishop;
        this.movementPatterns.push(new MovementPattern(owner,1,1,null,true));
        this.movementPatterns.push(new MovementPattern(owner,-1,1,null,true));
        this.movementPatterns.push(new MovementPattern(owner,1,-1,null,true));
        this.movementPatterns.push(new MovementPattern(owner,-1,-1,null,true));


        this.capturePatterns.push(new CapturePattern(owner,1,1,null,true));
        this.capturePatterns.push(new CapturePattern(owner,-1,1,null,true));
        this.capturePatterns.push(new CapturePattern(owner,1,-1,null,true));
        this.capturePatterns.push(new CapturePattern(owner,-1,-1,null,true));
    }
    toAscii(): string {
        return this.owner == 0 ? 'B' : 'b';
    }

}