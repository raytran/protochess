import {Schema, type} from "@colyseus/schema";
import {BoardLocation} from "./BoardLocation";
import {MovementPattern} from "./MovementPattern";
import {GameState} from "./GameState";
import {Movement} from "./Movement";
import {PieceType} from "./PieceType";
import {CapturePattern} from "./CapturePattern";
import shortid from "shortid";

export abstract class Piece extends Schema{
    @type("number")
    owner:number; //who owns this piece? 0 for white, 1 for black 2......
    @type(BoardLocation)
    location:BoardLocation;
    @type("string")
    pieceTypeStr:string;
    @type("string")
    id:string;
    @type("boolean")
    movedBefore:boolean;
    //The movement patterns that this piece uses
    movementPatterns:MovementPattern[];
    capturePatterns:CapturePattern[];
    pieceType:PieceType;
    constructor(owner:number,location:BoardLocation) {
        super();
        this.movedBefore = false;
        this.id = shortid.generate();
        this.pieceType = PieceType.Custom;
        this.owner = owner;
        this.movementPatterns = [];
        this.location = location;
        this.capturePatterns = [];
        this.pieceTypeStr = this.toAscii();
    }
    getPossibleMoves(gameState:GameState):Set<Movement>{
        if (this.location) {
            let returnMoves:Movement[] = [];
            for (let pattern of this.movementPatterns) {
                returnMoves.push(...Array.from(pattern.getPossibleMoves(this.location, gameState)));
            }
            for (let pattern of this.capturePatterns) {
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

    getLocation(){
        return this.location;
    }

    setLocation(loc:BoardLocation){
        this.movedBefore = true;
        this.location = loc;
    }
}