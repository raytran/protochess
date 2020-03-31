import React from "react";
import Draggable, {DraggableData, DraggableEvent} from "react-draggable";
import {Board, Piece} from "protochess-shared";
import {MapSchema} from "@colyseus/schema";
import {ChessPiece} from "./ChessPiece";

interface IProps {
    tileWidth: number,
    tileHeight: number,
    pieces: MapSchema<Piece>,
    playerNum: number,
    whosTurn: number,
    board: Board,
    inverted: boolean,
    onRequestMove: Function
}

export default class ChessPieces extends React.PureComponent<IProps> {
    constructor(props: IProps) {
        super(props);
        this.handleStop = this.handleStop.bind(this);
    }

    render() {
        return (
            <>
                {
                    Object.keys(this.props.pieces).map((key: string) => (
                        <Draggable
                            //@ts-ignore
                            key={key}
                            position={{x: 0, y: 0}}
                            disabled={this.props.pieces[key].owner != this.props.playerNum || this.props.pieces[key].owner != this.props.whosTurn}
                            onStop={this.handleStop}>
                            <span id={key} style={{"position": "absolute", "left": 0, "top": 0}}>
                                <ChessPiece pieceId={key}
                                            backgroundImage={'url(' + process.env.PUBLIC_URL + '/images/chess_pieces/' + this.props.pieces[key].pieceTypeStr + '.svg)'}
                                            x={this.props.pieces[key].x * this.props.tileWidth}
                                            y={this.props.inverted ? this.props.pieces[key].y * this.props.tileHeight : (this.props.board.height - this.props.pieces[key].y - 1) * this.props.tileHeight}
                                            height={this.props.tileHeight}
                                            width={this.props.tileWidth}/>
                            </span>

                        </Draggable>
                    ))}

            </>
        );
    }

    private handleStop(e: DraggableEvent, data: DraggableData) {
        let dx = data.x;
        let dy = data.y;
        let key = data.node.id;
        let x = this.props.pieces[key].x * this.props.tileWidth;
        let y = this.props.inverted ? this.props.pieces[key].y * this.props.tileHeight : (this.props.board.height - this.props.pieces[key].y - 1) * this.props.tileHeight;
        x += dx;
        y += dy;
        let requestX = Math.round(x / this.props.tileWidth);
        let requestY = Math.round(y / this.props.tileHeight);
        if (!this.props.inverted) {
            requestY = this.props.board.height - requestY - 1;
        }
        this.props.onRequestMove({
            startX: this.props.pieces[key].x,
            startY: this.props.pieces[key].y,
            endX: requestX,
            endY: requestY,
            id: data.node.id
        });
    }

}

