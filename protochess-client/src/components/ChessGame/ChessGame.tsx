import React, {Component, Ref} from "react";
import {GameState, Piece} from "protochess-shared";
import {ColorConstants} from "./ColorConstants";
import ChessBoard from "./ChessBoard";
import ChessPieces from "./ChessPieces";

interface IProps {
    width: number,
    height: number,
    gameState: GameState,
    onRequestMove: Function,
    inverted: boolean
}

interface IState {
    tileWidth: number,
    tileHeight: number
}

export default class ChessGame extends Component<IProps, IState> {
    private localChessPieces: Map<string, Piece>;
    private lastMoveHighlightBuffer: any[];
    private chessPieces: Ref<ChessPieces>;
    private chessBoard: Ref<ChessBoard>;

    constructor(props: IProps) {
        super(props);

        this.lastMoveHighlightBuffer = [];
        this.chessPieces = React.createRef();
        this.chessBoard = React.createRef();
        let tileWidth = this.props.width / this.props.gameState.board.width;
        let tileHeight = this.props.height / this.props.gameState.board.height;
        this.state = {tileWidth: tileWidth, tileHeight: tileHeight};
        this.localChessPieces = new Map<string, Piece>();
        for (let id in this.props.gameState.pieces) {
            const piece: Piece = this.props.gameState.pieces[id];
            this.localChessPieces.set(piece.id, piece.clone());
        }
        console.log(this.localChessPieces);
        this.onRequestMove = this.onRequestMove.bind(this);
    }

    componentDidMount(): void {
        console.log(this.props.inverted);
    }

    updatePiece(piece: Piece) {
        console.log(this.localChessPieces.get(piece.id));
        let oldPiece = this.localChessPieces.get(piece.id);
        while (this.lastMoveHighlightBuffer.length > 0) {
            let removed = this.lastMoveHighlightBuffer.shift();

            if (this.getTileHighlight(removed.x, removed.y) == ColorConstants.TO_HIGHLIGHT_COLOR ||
                this.getTileHighlight(removed.x, removed.y) == ColorConstants.FROM_HIGHLIGHT_COLOR) {
                this.setTileHighlight(removed.x, removed.y, ColorConstants.TRANSPARENT);
            }
        }
        if (oldPiece) {
            let x = oldPiece.location.x;
            let y = oldPiece.location.y;

            this.setTileHighlight(x, y, ColorConstants.FROM_HIGHLIGHT_COLOR);
            this.lastMoveHighlightBuffer.push({x: x, y: y});
            this.localChessPieces.set(piece.id, piece.clone());
        }

        let x = piece.location.x;
        let y = piece.location.y;
        this.setTileHighlight(x, y, ColorConstants.TO_HIGHLIGHT_COLOR);
        this.lastMoveHighlightBuffer.push({x: x, y: y})
    }

    deletePiece(piece: Piece) {
        //@ts-ignore
        this.chessPieces.current.deletePiece(piece);
    }

    lockAllPieces() {
        //@ts-ignore
        this.chessPieces.current.lockAllPieces()
    }

    unlockPieces(playerNum: number) {
        //@ts-ignore
        this.chessPieces.current.unlockPieces(playerNum);
    }

    getTileHighlight(x: number, y: number) {
        //@ts-ignore
        return this.chessBoard.current.getTileHighlight(x, y);
    }

    setTileHighlight(x: number, y: number, color: string) {
        //@ts-ignore
        this.chessBoard.current.setTileHighlight(x, y, color)
    }

    displayWinner(playerNum: number) {
    }

    render() {
        return (
            <div style={{width: this.props.width, height: this.props.height, position: 'relative', left: 0, top: 0}}>
                <ChessBoard
                    ref={this.chessBoard}
                    inverted={this.props.inverted}
                    tileWidth={this.state.tileWidth}
                    tileHeight={this.state.tileHeight}
                    board={this.props.gameState.board}/>

                <ChessPieces
                    ref={this.chessPieces}
                    tileWidth={this.state.tileWidth}
                    tileHeight={this.state.tileHeight}
                    inverted={this.props.inverted}
                    board={this.props.gameState.board}
                    pieces={this.props.gameState.pieces}
                    onRequestMove={this.onRequestMove}
                />
            </div>
        );
    }

    onRequestMove(move: { x: number; y: number; id: any }) {
        console.log(move);
        this.props.onRequestMove(move);
    }

}

