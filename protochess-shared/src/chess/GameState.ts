import {ArraySchema, MapSchema, Schema, type} from "@colyseus/schema";
import {Piece} from "./Piece";
import {Movement} from "./Movement";
import {Board} from "./Board";
import {BoardLocation} from "./BoardLocation";
import {PieceType} from "./PieceType";

export class GameState extends Schema{
    @type("number")
    whosTurn:number; //who's turn is it? --0 for white 1 for black 2.3..etc
    @type("number")
    numPlayers:number;
    @type("boolean")
    useChecks:boolean;
    @type("boolean")
    useCheckmates:boolean;
    @type(Board)
    board:Board;
    @type([Piece])
    pieces: ArraySchema<Piece>;

    private piecesInternal:Set<Piece>; //copy of pieces used for temporary modifications
    private kingMap:Map<number,Piece>;
    constructor(pieces:Set<Piece>,b:Board) {
        super();
        this.useChecks = true;
        this.useCheckmates = true;
        this.kingMap = new Map<number, Piece>();
        this.pieces = new ArraySchema<Piece>();
        for (let piece of pieces){
            this.pieces.push(piece);

            if (piece.pieceType == PieceType.King){
                this.kingMap.set(piece.owner,piece);
            }
        }

        this.piecesInternal = pieces;
        this.board = b;
        this.whosTurn = 0;
        this.numPlayers = 2;
    }

    //Takes a turn, returning true if valid; false otherwise
    takeTurn(movement:Movement):boolean{
        //Check if this move is on the board
        if (!this.board.onBoard(movement.end) || !this.board.onBoard(movement.start))
            return false;

        //Look for target piece
        let targetPiece = null;
        for (let piece of this.getPiecesOfPlayer(this.whosTurn)){
            if (piece.location.x == movement.start.x && piece.location.y == movement.start.y){
                targetPiece = piece;
                break;
            }
        }

        if (!targetPiece) return false;
        //Correct color
        if (this.whosTurn != targetPiece.owner) return false;



        //Check if this moves is in the player's possible moves
        let isValid = false;
        for (let move of this.getValidMoves(this.whosTurn).get(targetPiece)!){
            if (move.end.x == movement.end.x
            && move.end.y == movement.end.y
            && move.start.x == movement.start.x
            && move.start.y == movement.start.y){
                isValid = true;
            }
        }

        if (isValid){

            //Make the move
            if(movement.capturedPiece){
                console.log("Piece captured");
                this.piecesInternal.delete(movement.capturedPiece);
                let deleteIndex = this.pieces.indexOf(movement.capturedPiece);
                delete this.pieces[deleteIndex];
            }

            targetPiece.location = movement.end;

            //Next player's turn
            this.whosTurn = (this.whosTurn + 1) % this.numPlayers;
            //Check if the other player is in checkmate
            if (this.getValidMoves(this.whosTurn).size == 0 ){
                console.log("OTHER PLAYER CHECKMATE")
            }
            return true;
        }else{
            return false;
        }
    }

    //Returns a map of valid moves for this player
    getValidMoves(playerNumber:number){
        let possibleMoves = new Map<Piece,Set<Movement>>();
        let inCheck = false;
        if (this.getChecks().has(playerNumber)){
            inCheck = true;
        }
        for (let piece of this.getPiecesOfPlayer(playerNumber)){
            if (!inCheck){
                possibleMoves.set(piece,piece.getPossibleMoves(this));
            }else{
                //Check if this move would bring this player out of check
                //Simulate move
                let oldLocation = piece.location;
                for (let possibleMove of piece.getPossibleMoves(this)){
                    piece.location = possibleMove.end;

                    if(possibleMove.capturedPiece){
                        this.piecesInternal.delete(possibleMove.capturedPiece);
                    }

                    if (!this.getChecks().has(playerNumber)){
                        //Not in check after this move, add to list
                        if (possibleMoves.get(piece)){
                            possibleMoves.get(piece)!.add(possibleMove);
                        }else {
                            let newSet = new Set<Movement>();
                            newSet.add(possibleMove);
                            possibleMoves.set(piece,newSet);
                        }
                    }
                    //Undo move
                    if(possibleMove.capturedPiece){
                        this.piecesInternal.add(possibleMove.capturedPiece);
                    }

                    piece.location = oldLocation;
                }
            }
        }
        return possibleMoves;

    }

    getPiecesOfPlayer(playerNum:number):Set<Piece>{
        let returnSet = new Set<Piece>();
        for (let piece of this.pieces) {
            if (piece.owner == playerNum){
                returnSet.add(piece);
            }
        }
        return returnSet;
    }

    //Returns a set of players in check
    getChecks(){
        let playersInCheck = [];
        for (let piece of this.piecesInternal){
            for (let move of piece.getPossibleMoves(this)){
                if(move.capturedPiece?.pieceType == PieceType.King){
                    playersInCheck.push(move.capturedPiece.owner);
                }
            }
        }
        return new Set(playersInCheck);
    }


    //returns the board as ascii representation
    toAscii(){
        let returnString = "";
        if (this.board.height && this.board.width) {
            for (let i = this.board.height - 1; i >= 0; i--) {
                for (let j = 0; j < this.board.width; j++) {
                    let pieceHere = null;
                    for (let piece of this.piecesInternal){
                        if (piece.location!.x == j && piece.location!.y == i ){
                            pieceHere = piece;
                            break;
                        }
                    }

                    if (pieceHere) {
                        returnString += pieceHere.toAscii();
                    }else returnString += ' ';
                }
                returnString+=' '+i+'\n';
            }
        }
        return returnString;
    }

    onBoard(boardLocation: BoardLocation) {
        return this.board.onBoard(boardLocation);
    }

    //Returns the piece at this board location
    pieceAt(newLoc: BoardLocation) {
        for (let piece of this.piecesInternal){
            if (newLoc.x == piece.location.x && newLoc.y == piece.location.y){
                return piece;
            }
        }
        return null;
    }
}