import {Piece} from "../Piece";
import {MovementPattern} from "../MovementPattern";
import {BoardLocation} from "../BoardLocation";
import {PieceType} from "../PieceType";
import {CapturePattern} from "../CapturePattern";

export class Queen extends Piece {
    constructor(owner: number, location: BoardLocation) {
        super(owner, location);
        this.pieceType = PieceType.Queen;
        this.movementPatterns.push(new MovementPattern(owner, 1, 0, null, true));
        this.movementPatterns.push(new MovementPattern(owner, -1, 0, null, true));
        this.movementPatterns.push(new MovementPattern(owner, 0, 1, null, true));
        this.movementPatterns.push(new MovementPattern(owner, 0, -1, null, true));
        this.movementPatterns.push(new MovementPattern(owner, 1, 1, null, true));
        this.movementPatterns.push(new MovementPattern(owner, -1, 1, null, true));
        this.movementPatterns.push(new MovementPattern(owner, 1, -1, null, true));
        this.movementPatterns.push(new MovementPattern(owner, -1, -1, null, true));

        this.capturePatterns.push(new CapturePattern(owner, 1, 0, null, true));
        this.capturePatterns.push(new CapturePattern(owner, -1, 0, null, true));
        this.capturePatterns.push(new CapturePattern(owner, 0, 1, null, true));
        this.capturePatterns.push(new CapturePattern(owner, 0, -1, null, true));
        this.capturePatterns.push(new CapturePattern(owner, 1, 1, null, true));
        this.capturePatterns.push(new CapturePattern(owner, -1, 1, null, true));
        this.capturePatterns.push(new CapturePattern(owner, 1, -1, null, true));
        this.capturePatterns.push(new CapturePattern(owner, -1, -1, null, true));
    }

    toAscii(): string {
        return this.owner == 0 ? 'Q' : 'q';
    }

}