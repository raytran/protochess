import {fabric} from "fabric";
import {GameState} from "protochess-shared";
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

const PieceLoader = {
    loadPieces(canvas:fabric.Canvas, gameState:GameState,tileWidth:number,tileHeight:number){
        for (let piece of gameState.pieces){
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


            fabric.loadSVGFromURL(svgURL, function(objects, options) {
                var obj = fabric.util.groupSVGElements(objects, options);
                obj.left = 0;
                obj.top = 0;
                obj.originX = "center";
                obj.originY = "center";
                if (svgURL !== wPawn && svgURL !== bPawn){
                    obj!.scaleToHeight(tileHeight);
                    obj!.scaleToWidth(tileWidth);
                }else{
                    obj!.scaleToHeight(tileHeight/2);
                    obj!.scaleToWidth(tileWidth/2);
                }

                let x = piece.location.x;
                let y= piece.location.y;

                let rect2 = new fabric.Rect({
                    left : 0,
                    top : 0,
                    width : tileWidth,
                    height : tileHeight,
                    fill : 'rgba(0,0,0,0)',
                    originX:'center',
                    originY:'center'
                });
                let group = new fabric.Group([rect2,obj]);
                group.top = y*tileHeight;
                group.left = x*tileWidth;
                group.hasControls=false;
                group.hoverCursor='default';
                canvas.add(group);



                canvas.renderAll();
            });

        }
    }
};

export default PieceLoader;
