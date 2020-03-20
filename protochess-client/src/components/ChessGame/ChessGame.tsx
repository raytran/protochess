import React, {Component} from "react";
import {GameState,Piece} from "protochess-shared";
import {fabric} from "fabric";
import "./ChessGame.css";
import Board from "./Board";
import PieceHandler from "./PieceHandler";

interface IProps {
    gameState:GameState,
    requestMove:Function,
    inverted:boolean | null,
    playerNum:number | null
}
export default class ChessGame extends Component<IProps> {
    private tileMap:Map<string,fabric.Object> = new Map();
    private pieceMap:Map<string,fabric.Object> = new Map();
    private tileWidth:number = 0;
    private tileHeight:number = 0;
    private canvas:fabric.Canvas | null = null;
    componentDidUpdate(prevProps: Readonly<IProps>, prevState: Readonly<{}>, snapshot?: any): void {
        if (this.props.playerNum !== null
            && this.props.inverted !== null
            && this.props.inverted !== prevProps.inverted){
            PieceHandler.deleteAllPieces();
            Board.deleteAll();
            //Maps X#Y# to a fabric rect
            this.tileMap = Board.loadBoard(this.props.inverted, this.props.gameState);

            //Maps pieceID to fabric group
            this.pieceMap = PieceHandler.loadPieces(this.props.playerNum,this.props.inverted, this, this.props.gameState);
        }
    }

    componentDidMount(): void {
        let canvas = new fabric.Canvas('myCanvas');
        this.canvas = canvas;
        canvas.preserveObjectStacking = true;
        canvas.setHeight(500);
        canvas.setWidth(500);
        canvas.setBackgroundColor('rgba(255, 255, 255, 1)', canvas.renderAll.bind(canvas));
        canvas.selection = false;

        canvas.on({
            'object:moving': function(e) {
                e!.target!.opacity = 0.6;
            },
            'object:modified': function(e) {
                e!.target!.opacity = 1;
            }
        });

        this.tileWidth = canvas.getWidth()/this.props.gameState.board.width;
        this.tileHeight = canvas.getHeight()/this.props.gameState.board.height;
        Board.init(canvas,this.tileWidth,this.tileHeight);
        PieceHandler.init(canvas,this.tileWidth,this.tileHeight);

    }

    updatePiece(piece:Piece) {
        PieceHandler.updatePiece(this.props.gameState,piece);
    }

    deletePiece(piece:Piece){
        PieceHandler.deletePiece(piece);
    }

    lockAllPieces(playerNum:number){
        PieceHandler.lockAllPieces();
    }

    unlockPieces(playerNum:number){
        PieceHandler.unlockPieces(playerNum);
    }

    render() {
        return (
            <div>
                <canvas id="myCanvas"/>
            </div>
        );
    }

    requestMove(move: {x: number; y: number; id: any}) {
        this.props.requestMove(move);
    }

}

