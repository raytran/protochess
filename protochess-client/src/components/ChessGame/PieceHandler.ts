import {fabric} from "fabric";
import {GameState,Piece} from "protochess-shared";
import wKing from "./Assets/chess_pieces/Chess_klt45.svg";
import bKing from "./Assets/chess_pieces/Chess_kdt45.svg";
import wPawn from "./Assets/chess_pieces/Chess_plt45.svg";
import bPawn from "./Assets/chess_pieces/Chess_pdt45.svg";
import wQueen from "./Assets/chess_pieces/Chess_qlt45.svg";
import bQueen from "./Assets/chess_pieces/Chess_qdt45.svg";
import wRook from "./Assets/chess_pieces/Chess_rlt45.svg";
import bRook from "./Assets/chess_pieces/Chess_rdt45.svg";
import wKnight from "./Assets/chess_pieces/Chess_nlt45.svg";
import bKnight from "./Assets/chess_pieces/Chess_ndt45.svg";
import wBishop from "./Assets/chess_pieces/Chess_blt45.svg";
import bBishop from "./Assets/chess_pieces/Chess_bdt45.svg";
import ChessGame from "./ChessGame";

const PieceHandler = {
    playerNumPieceMap: new Map<number,Set<Piece>>(),
    pieceMap: new Map<string,fabric.Object>(),
    inverted: false,
    canvas: null,
    tileWidth: -1,
    tileHeight: -1,
    init(canvas:fabric.Canvas,tileWidth:number,tileHeight:number){
        //@ts-ignore
        this.canvas = canvas;
        this.tileWidth = tileWidth;
        this.tileHeight = tileHeight;
    },
    loadPieces(playerNum:number,inverted:boolean,chessGame:ChessGame, gameState:GameState){
        this.inverted = inverted;
        let this_ = this;
        for (let pieceId in gameState.pieces){
            let piece = gameState.pieces[pieceId];

            if (this.playerNumPieceMap.has(piece.owner)){
                this.playerNumPieceMap.get(piece.owner)!.add(piece);
            }else{
                let newSet = new Set<Piece>();
                newSet.add(piece);
                this.playerNumPieceMap.set(piece.owner,newSet);
            }
            let svgURL = wKing;
            switch (piece.pieceTypeStr){
                case 'k':
                    svgURL = bKing;
                    break;
                case 'K':
                    svgURL = wKing;
                    break;
                case 'q':
                    svgURL = bQueen;
                    break;
                case 'Q':
                    svgURL = wQueen;
                    break;
                case 'b':
                    svgURL = bBishop;
                    break;
                case 'B':
                    svgURL = wBishop;
                    break;
                case 'n':
                    svgURL = bKnight;
                    break;
                case 'N':
                    svgURL = wKnight;
                    break;
                case 'r':
                    svgURL = bRook;
                    break;
                case 'R':
                    svgURL = wRook;
                    break;
                case 'p':
                    svgURL = bPawn;
                    break;
                case 'P':
                    svgURL = wPawn;
                    break;
            }


            let this_ = this;
            fabric.loadSVGFromURL(svgURL, function(objects, options) {
                var obj = fabric.util.groupSVGElements(objects, options);
                obj.left = 0;
                obj.top = 0;
                obj.originX = "center";
                obj.originY = "center";
                if (svgURL !== wPawn && svgURL !== bPawn){
                    obj!.scaleToHeight(this_.tileHeight);
                    obj!.scaleToWidth(this_.tileWidth);
                }else{
                    obj!.scaleToHeight(this_.tileHeight/2);
                    obj!.scaleToWidth(this_.tileWidth/2);
                }

                let x = piece.location.x;
                let y= piece.location.y;

                if (!inverted){
                    y = gameState.board.height - y - 1;
                }

                let rect2 = new fabric.Rect({
                    left : 0,
                    top : 0,
                    width : this_.tileWidth,
                    height : this_.tileHeight,
                    fill : 'rgba(0,0,0,0)',
                    originX:'center',
                    originY:'center'
                });
                let group = new fabric.Group([rect2,obj]);
                group.top = y*this_.tileHeight;
                group.left = x*this_.tileWidth;
                group.hasControls=false;
                group.hoverCursor='default';
                this_.pieceMap.set(piece.id,group);

                if (playerNum !== piece.owner
                    || playerNum !== gameState.whosTurn){
                    group.evented = false;
                }

                let oldTop = 0;
                let oldLeft = 0;
                group.on({
                    "mousedown":function(){
                        //Record previous loc
                        oldTop = group.top!;
                        oldLeft = group.left!;
                    },

                    "modified":function(){
                        let requestX = Math.round(group.left!/this_.tileWidth);
                        let requestY = Math.round(group.top!/this_.tileHeight);

                        if (!inverted){
                            requestY = gameState.board.height - requestY - 1;
                        }
                        var promise = new Promise(function(resolve, reject) {
                            group!.set('left',oldLeft);
                            group!.set('top',oldTop);
                            group!.setCoords();
                            resolve("Stuff worked!");
                        });

                        promise.then(()=>{
                            chessGame.requestMove({'id':piece.id,x:requestX,y:requestY});
                        });
                    }

                });
                group.hasBorders = false;
                //@ts-ignore
                this_.canvas.add(group);
                //@ts-ignore
                this_.canvas.renderAll();
            });

        }
        return this.pieceMap;
    },

    updatePiece(gameState:GameState,piece:Piece){
        if (this.pieceMap.has(piece.id)){
            let group = this.pieceMap.get(piece.id);
            let y = piece.location.y;
            let x = piece.location.x;

            if (!this.inverted){
                y = gameState.board.height - y - 1;
            }
            group!.set('left',x*this.tileWidth);
            group!.set('top',y*this.tileHeight);
            group!.setCoords();
            //@ts-ignore
            this.canvas.renderAll();
        }
    },

    lockAllPieces(){
        for (let key of this.pieceMap.keys()) {
            let value = this.pieceMap.get(key);
            value!.evented = false;
        }
    },

    lockPieces(playerNum:number){
        if (this.playerNumPieceMap.has(playerNum)){
            for (let piece of this.playerNumPieceMap.get(playerNum)!){
                let group = this.pieceMap.get(piece.id);
                group!.evented = false;
            }
        }
    },

    unlockPieces(playerNum:number){
        if (this.playerNumPieceMap.has(playerNum)){
            for (let piece of this.playerNumPieceMap.get(playerNum)!){
                let group = this.pieceMap.get(piece.id);
                if (group) group!.evented = true;
            }
        }
    },

    deletePiece(piece:Piece){
        if (this.pieceMap.has(piece.id)){
            let group = this.pieceMap.get(piece.id);
            //@ts-ignore
            this.canvas.remove(group!);
            //@ts-ignore
            this.canvas.renderAll();
            this.pieceMap.delete(piece.id);
            this.playerNumPieceMap.get(piece.owner)!.delete(piece);
        }
    },

    deleteAllPieces(){
        for (let key of this.pieceMap.keys()) {
            let value = this.pieceMap.get(key);
            //@ts-ignore
            this.canvas.remove(value!);
            this.pieceMap.delete(key);
        }
    }

};

export default PieceHandler;
