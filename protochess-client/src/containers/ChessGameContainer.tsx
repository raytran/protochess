import React, {Component, Ref} from "react";
import * as Colyseus from "colyseus.js";
import {Piece} from "protochess-shared";
import ChessGame from '../components/ChessGame/ChessGame';
import ClientHandler from '../ClientHandler';
import {ColorConstants} from "../components/ChessGame/ColorConstants";

interface IProps {
    width: number,
    height: number
}

interface IState {
    inverted: boolean | null,
    playerNum: number | null,
    propsReady: boolean,
    gameState: any
}

export default class ChessGameContainer extends Component<IProps, IState> {
    private chessGame: Ref<ChessGame>;

    constructor(props: IProps) {
        super(props);
        this.chessGame = React.createRef();
        this.onPieceChange = this.onPieceChange.bind(this);
        this.onPlayerNum = this.onPlayerNum.bind(this);
        this.onPlayerChange = this.onPlayerChange.bind(this);
        this.onGameStateChange = this.onGameStateChange.bind(this);
        this.state = {
            gameState: ClientHandler.getRoom()!.state.gameState,
            propsReady: false,
            inverted: null,
            playerNum: null
        };
    }

    componentDidMount(): void {
        ClientHandler.addPlayerNumListener(this.onPlayerNum);
        ClientHandler.addGameStateListener(this.onGameStateChange);
        ClientHandler.addPlayerChangeListener(this.onPlayerChange);
        ClientHandler.addPieceChangeListener(this.onPieceChange);
    }

    componentWillUnmount(): void {
        ClientHandler.removePieceChangeListener(this.onPieceChange);
        ClientHandler.removePlayerNumListener(this.onPlayerNum);
        ClientHandler.removePlayerChangeListener(this.onPlayerChange);
        ClientHandler.removeGameStateListener(this.onGameStateChange);
    }

    render() {
        if (ClientHandler.getRoom() && this.state.propsReady) {
            return (
                <ChessGame width={this.props.width}
                           height={this.props.height}
                           ref={this.chessGame}
                           playerNum={this.state.playerNum!}
                           inverted={this.state.inverted!}
                           onRequestMove={this.onRequestMove}
                           gameState={this.state.gameState}/>
            );
        } else {
            return null;
        }
    }

    private setKingTileColor(playerNum: number, color: string) {
        for (let id in ClientHandler.getRoom()!.state.gameState.pieces) {
            const piece: Piece = ClientHandler.getRoom()!.state.gameState.pieces[id];
            if (piece.pieceTypeStr == 'k' || piece.pieceTypeStr == "K") {
                //@ts-ignore
                if (piece.owner == playerNum) {
                    //@ts-ignore
                    if (this.chessGame.current) {
                        //@ts-ignore
                        this.chessGame.current.setTileHighlight(piece.x, piece.y, color);
                    }
                }
            }
        }
    }

    private onPlayerChange(changes: Colyseus.DataChange[]) {
        if (changes) {
            //@ts-ignore
            if (changes['inCheck']) {
                //@ts-ignore
                this.setKingTileColor(changes['playerNum'], ColorConstants.IN_CHECK_HIGHLIGHT_COLOR);
            } else {
                //@ts-ignore
                this.setKingTileColor(changes['playerNum'], 'rgba(0,0,0,0)');
            }
        }
    }

    private onPlayerNum() {
        let playerNum = ClientHandler.getPlayerNum();
        console.log("MY PLAYER NUM IS:");
        console.log(playerNum);
        if (playerNum !== null && playerNum >= 0) {
            if (playerNum! % 2 == 1) {
                this.setState({propsReady: true, inverted: true, playerNum: playerNum})
            } else {
                this.setState({propsReady: true, inverted: false, playerNum: playerNum})
            }
        }
    }

    private onGameStateChange(changes: Colyseus.DataChange[]) {
        console.log(changes);
        this.setState({gameState: ClientHandler.getRoom()!.state.gameState})
        this.forceUpdate();
    }

    private onPieceChange(piece: Piece) {
        if (this.chessGame) {
            //@ts-ignore
            this.chessGame.current.updatePieceHighlighting(piece);
        }
    }

    private onRequestMove(move: { id: string, startX: number, startY: number, endX: number, endY: number }) {
        ClientHandler.requestMove(move);
    }
}

