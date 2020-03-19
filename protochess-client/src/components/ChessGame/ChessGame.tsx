import React, {Component} from "react";
import {GameState} from "protochess-shared";
import {fabric} from "fabric";
import "./ChessGame.css";
import Board from "./Board";
import PieceLoader from "./PieceLoader";

export default class ChessGame extends Component<{gameState:GameState}> {
    componentDidMount(): void {
        let canvas = new fabric.Canvas('myCanvas');
        canvas.preserveObjectStacking = true;
        canvas.setHeight(500);
        canvas.setWidth(500);
        canvas.setBackgroundColor('rgba(255, 255, 255, 1)', canvas.renderAll.bind(canvas));

        let tileWidth = canvas.getWidth()/this.props.gameState.board.width;
        let tileHeight = canvas.getHeight()/this.props.gameState.board.height;

        let tileMap = Board.loadBoard(canvas,this.props.gameState,tileWidth,tileHeight);
        PieceLoader.loadPieces(canvas,this.props.gameState,tileWidth,tileHeight);
    }

    render() {
        return (
            <div>
                <canvas id="myCanvas"/>
            </div>
        );
    }
}

