import React, {Component} from "react";
import {Map as IMap} from "immutable";
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
    board: Board,
    inverted: boolean,
    onRequestMove: Function
}

interface IState {
    disabledDragMap: IMap<string, boolean>
}

export default class ChessPieces extends Component<IProps, IState> {
    private pieceIds: string[] = [];

    constructor(props: IProps) {
        super(props);
        this.handleStop = this.handleStop.bind(this);
        let disabledDragMap = new Map<string, boolean>();
        for (let id in this.props.pieces) {
            const piece: Piece = this.props.pieces[id];
            this.pieceIds.push(id);
            disabledDragMap.set(piece.id, false);
        }

        //Make maps immutable
        this.state = {
            //@ts-ignore
            disabledDragMap: new IMap(disabledDragMap),
        }
    }

    render() {
        return (
            <>
                {
                    this.pieceIds.map((key: string) => (
                        <Draggable
                            //@ts-ignore
                            key={key}
                            position={{x: 0, y: 0}}
                            disabled={this.state.disabledDragMap.get(key)}
                            onStop={this.handleStop}>
                            <span id={key} style={{"position": "absolute", "left": 0, "top": 0}}>
                                <ChessPiece pieceId={key}
                                            svgItem={this.getSVGFromTypeStr(this.props.pieces[key].pieceTypeStr)}
                                            x={this.props.pieces[key].location.x * this.props.tileWidth}
                                            y={this.props.inverted ? this.props.pieces[key].location.y * this.props.tileHeight : (this.props.board.height - this.props.pieces[key].location.y - 1) * this.props.tileHeight}
                                            height={this.props.tileHeight}
                                            width={this.props.tileWidth}/>
                            </span>

                        </Draggable>
                    ))}

            </>
        );
    }

    lockAllPieces() {
        let newMap = new Map<string, boolean>();
        this.state.disabledDragMap.forEach((value, key) => {
            newMap.set(key, true);
        });

        //@ts-ignore
        this.setState({disabledDragMap: new IMap(newMap)});
    }

    unlockPieces(playerNum: number) {
        let newMap = new Map<string, boolean>();
        this.state.disabledDragMap.forEach((value, key) => {
            if (this.props.pieces[key]!.owner == playerNum) {
                newMap.set(key, false);
            } else {
                newMap.set(key, true);
            }
        });
        //@ts-ignore
        this.setState({disabledDragMap: new IMap(newMap)});
    }

    deletePiece(piece: Piece) {
        if (this.pieceIds.indexOf(piece.id)) {
            this.pieceIds.splice(this.pieceIds.indexOf(piece.id), 1)
        }
        this.setState({
            disabledDragMap:
                this.state.disabledDragMap.delete(piece.id)
        });
    }

    private getSVGFromTypeStr(pieceTypeStr: string): React.ReactElement {
        let svgItem = <WKing viewBox={'0 0 45 45'}/>;
        switch (pieceTypeStr) {
            case 'k':
                svgItem = <BKing viewBox={"0 0 45 45"}/>;
                break;
            case 'K':
                svgItem = <WKing viewBox={"0 0 45 45"}/>;
                break;
            case 'q':
                svgItem = <BQueen viewBox={"0 0 45 45"}/>;
                break;
            case 'Q':
                svgItem = <WQueen viewBox={"0 0 45 45"}/>;
                break;
            case 'b':
                svgItem = <BBishop viewBox={"0 0 45 45"}/>;
                break;
            case 'B':
                svgItem = <WBishop viewBox={"0 0 45 45"}/>;
                break;
            case 'n':
                svgItem = <BKnight viewBox={"0 0 45 45"}/>;
                break;
            case 'N':
                svgItem = <WKnight viewBox={"0 0 45 45"}/>;
                break;
            case 'r':
                svgItem = <BRook viewBox={"0 0 45 45"}/>;
                break;
            case 'R':
                svgItem = <WRook viewBox={"0 0 45 45"}/>;
                break;
            case 'p':
                svgItem = <BPawn viewBox={"-0.57 1 45 45"}/>;
                break;
            case 'P':
                svgItem = <WPawn viewBox={"-0.57 1 45 45"}/>;
                break;
        }
        return svgItem;
    }

    private handleStop(e: DraggableEvent, data: DraggableData) {
        let dx = data.x;
        let dy = data.y;
        let key = data.node.id;
        let x = this.props.pieces[key].location.x * this.props.tileWidth;
        let y = this.props.inverted ? this.props.pieces[key].location.y * this.props.tileHeight : (this.props.board.height - this.props.pieces[key].location.y - 1) * this.props.tileHeight;
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

