import React from "react";
import Draggable, {DraggableData, DraggableEvent} from "react-draggable";
import {Board, Piece} from "protochess-shared";
import {ReactComponent as WKing} from "./Assets/chess_pieces/Chess_klt45.svg";
import {ReactComponent as BKing} from "./Assets/chess_pieces/Chess_kdt45.svg";
import {ReactComponent as WPawn} from "./Assets/chess_pieces/Chess_plt45.svg";
import {ReactComponent as BPawn} from "./Assets/chess_pieces/Chess_pdt45.svg";
import {ReactComponent as WQueen} from "./Assets/chess_pieces/Chess_qlt45.svg";
import {ReactComponent as BQueen} from "./Assets/chess_pieces/Chess_qdt45.svg";
import {ReactComponent as WRook} from "./Assets/chess_pieces/Chess_rlt45.svg";
import {ReactComponent as BRook} from "./Assets/chess_pieces/Chess_rdt45.svg";
import {ReactComponent as WKnight} from "./Assets/chess_pieces/Chess_nlt45.svg";
import {ReactComponent as BKnight} from "./Assets/chess_pieces/Chess_ndt45.svg";
import {ReactComponent as WBishop} from "./Assets/chess_pieces/Chess_blt45.svg";
import {ReactComponent as BBishop} from "./Assets/chess_pieces/Chess_bdt45.svg";
import {MapSchema} from "@colyseus/schema";
import {ChessPiece} from "./ChessPiece";

interface IProps {
    tileWidth: number,
    tileHeight: number,
    pieces: MapSchema<Piece>,
    playerNum: number,
    board: Board,
    inverted: boolean,
    onRequestMove: Function
}

export default class ChessPieces extends React.PureComponent<IProps> {
    private wking = <WKing viewBox={'0 0 45 45'}/>;
    private bking = <BKing viewBox={"0 0 45 45"}/>;
    private bqueen = <BQueen viewBox={"0 0 45 45"}/>;
    private wqueen = <WQueen viewBox={"0 0 45 45"}/>;
    private bbishop = <BBishop viewBox={"0 0 45 45"}/>;
    private wbishop = <WBishop viewBox={"0 0 45 45"}/>;
    private bknight = <BKnight viewBox={"0 0 45 45"}/>;
    private wknight = <WKnight viewBox={"0 0 45 45"}/>;
    private brook = <BRook viewBox={"0 0 45 45"}/>;
    private wrook = <WRook viewBox={"0 0 45 45"}/>;
    private bpawn = <BPawn viewBox={"-0.57 1 45 45"}/>;
    private wpawn = <WPawn viewBox={"-0.57 1 45 45"}/>;

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
                            disabled={this.props.pieces[key].owner != this.props.playerNum}
                            onStop={this.handleStop}>
                            <span id={key} style={{"position": "absolute", "left": 0, "top": 0}}>
                                <ChessPiece pieceId={key}
                                            svgItem={this.getSVGFromTypeStr(this.props.pieces[key].pieceTypeStr)}
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

    private getSVGFromTypeStr(pieceTypeStr: string): React.ReactElement {
        let svgItem = this.wking;
        switch (pieceTypeStr) {
            case 'k':
                svgItem = this.bking;
                break;
            case 'K':
                svgItem = this.wking;
                break;
            case 'q':
                svgItem = this.bqueen;
                break;
            case 'Q':
                svgItem = this.wqueen;
                break;
            case 'b':
                svgItem = this.bbishop;
                break;
            case 'B':
                svgItem = this.wbishop;
                break;
            case 'n':
                svgItem = this.bknight;
                break;
            case 'N':
                svgItem = this.wknight;
                break;
            case 'r':
                svgItem = this.brook;
                break;
            case 'R':
                svgItem = this.wrook;
                break;
            case 'p':
                svgItem = this.bpawn;
                break;
            case 'P':
                svgItem = this.wpawn;
                break;
        }
        return svgItem;
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
        this.props.onRequestMove({x: requestX, y: requestY, id: data.node.id});
    }

}

