import {Piece} from "./Piece";
import {King} from "./classic/King";
import {Queen} from "./classic/Queen";
import {Bishop} from "./classic/Bishop";
import {Pawn} from "./classic/Pawn";
import {Rook} from "./classic/Rook";
import {Knight} from "./classic/Knights";
import {BoardLocation} from "./BoardLocation";

export class PieceBuilder{
    //Returns a set of classic pieces
    static classicSet():Set<Piece>{
        let returnSet = new Set<Piece>();

        //Repeat for black and white
        for (let i=0;i<2;i++){
            let yLoc = 0;
            if (i==1) yLoc = 7;
            let iKing = new King(i,new BoardLocation(4,yLoc));
            let iQueen = new Queen(i,new BoardLocation(3,yLoc));
            let i1Knight = new Knight(i,new BoardLocation(1,yLoc));
            let i2Knight = new Knight(i,new BoardLocation(6,yLoc));
            let i1Bishop = new Bishop(i,new BoardLocation(2,yLoc));
            let i2Bishop = new Bishop(i,new BoardLocation(5,yLoc));
            let i1Rook = new Rook(i,new BoardLocation(0,yLoc));
            let i2Rook = new Rook(i,new BoardLocation(7,yLoc));

            for (let j=0;j<8;j++){
                let newPawn = new Pawn(i,new BoardLocation(j,i == 0 ? 1 : 6));
                returnSet.add(newPawn);
            }

            returnSet.add(iKing);
            returnSet.add(iQueen);
            returnSet.add(i1Knight);
            returnSet.add(i2Knight);
            returnSet.add(i1Bishop);
            returnSet.add(i2Bishop);
            returnSet.add(i1Rook);
            returnSet.add(i2Rook);
        }
        console.log(returnSet.size);
        return returnSet;
    }

}