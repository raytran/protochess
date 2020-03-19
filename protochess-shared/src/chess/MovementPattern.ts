import {Movement} from "./Movement";
import {BoardLocation} from "./BoardLocation";
import {GameState} from "./GameState";

//Defines how a piece can move (not capture)
export class MovementPattern {
    owner:number;
    blockable: boolean;
    dx:number;
    dy:number;
    stepMax:number | null;
    //Stepmax inclusive e.g. king moves has stepmax 1
    constructor(owner:number,dx:number,dy:number,stepMax:number | null,blockable:boolean) {
        this.owner=owner;
        this.blockable = blockable;
        this.dx = dx;
        this.dy = dy;
        this.stepMax = stepMax;
    }
    //Gets possible moves from a board location on a board
    getPossibleMoves(startLoc: BoardLocation, gameState:GameState):Set<Movement>{
        let possibleMoves = new Set<Movement>();
        let newX = startLoc.x;
        let newY = startLoc.y;
        let stepMax = this.stepMax ? this.stepMax : Math.max(gameState.board.width,gameState.board.height);
        for (let i=0;i<stepMax;i++){
            newX += this.dx;
            newY += this.dy;
            let newLoc = new BoardLocation(newX,newY)
            let newMove = new Movement(startLoc,newLoc);
            //Make sure this location is valid
            if(gameState.onBoard(newLoc)){
                let pieceHere = gameState.pieceAt(newLoc);
                if (pieceHere){
                    if (this.blockable) break;
                }else{
                    possibleMoves.add(newMove);
                }
            }
        }
        return possibleMoves;
    }
}

