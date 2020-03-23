import {Piece} from "../Piece";
import {MovementPattern} from "../MovementPattern";
import {BoardLocation} from "../BoardLocation";
import {PieceType} from "../PieceType";
import {CapturePattern} from "../CapturePattern";

export class King extends Piece {
    constructor(owner: number, location: BoardLocation) {
        super(owner, location);
        this.pieceType = PieceType.King;
        this.movementPatterns.push(new MovementPattern(owner, 1, 0, 1, true));
        this.movementPatterns.push(new MovementPattern(owner, -1, 0, 1, true));
        this.movementPatterns.push(new MovementPattern(owner, 0, 1, 1, true));
        this.movementPatterns.push(new MovementPattern(owner, 0, -1, 1, true));
        this.movementPatterns.push(new MovementPattern(owner, 1, 1, 1, true));
        this.movementPatterns.push(new MovementPattern(owner, -1, 1, 1, true));
        this.movementPatterns.push(new MovementPattern(owner, 1, -1, 1, true));
        this.movementPatterns.push(new MovementPattern(owner, -1, -1, 1, true));

        this.capturePatterns.push(new CapturePattern(owner, 1, 0, 1, true));
        this.capturePatterns.push(new CapturePattern(owner, -1, 0, 1, true));
        this.capturePatterns.push(new CapturePattern(owner, 0, 1, 1, true));
        this.capturePatterns.push(new CapturePattern(owner, 0, -1, 1, true));
        this.capturePatterns.push(new CapturePattern(owner, 1, 1, 1, true));
        this.capturePatterns.push(new CapturePattern(owner, -1, 1, 1, true));
        this.capturePatterns.push(new CapturePattern(owner, 1, -1, 1, true));
        this.capturePatterns.push(new CapturePattern(owner, -1, -1, 1, true));
    }

    toAscii(): string {
        return this.owner == 0 ? 'K' : 'k';
    }

}