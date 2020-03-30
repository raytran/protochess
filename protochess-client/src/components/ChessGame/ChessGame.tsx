import React, {Component, Ref} from "react";
import {GameState, Piece} from "protochess-shared";
import {ColorConstants} from "./ColorConstants";
import ChessBoard from "./ChessBoard";
import ChessPieces from "./ChessPieces";

interface IProps {
    width: number,
    height: number,
    gameState: GameState,
    playerNum: number,
    onRequestMove: Function,
    inverted: boolean
}

export default class ChessGame extends Component<IProps> {
    private localChessPieces: Map<string, Piece>;
    private lastMoveHighlightBuffer: any[];
    private chessPieces: Ref<ChessPieces>;
    private chessBoard: Ref<ChessBoard>;
    private tileWidth = this.props.width / this.props.gameState.board.width;
    private tileHeight = this.props.height / this.props.gameState.board.height;

    constructor(props: IProps) {
        super(props);
        this.lastMoveHighlightBuffer = [];
        this.chessPieces = React.createRef();
        this.chessBoard = React.createRef();
        this.localChessPieces = new Map<string, Piece>();
        for (let id in this.props.gameState.pieces) {
            const piece: Piece = this.props.gameState.pieces[id];
            this.localChessPieces.set(piece.id, piece.clone());
        }
    }

    updatePieceHighlighting(piece: Piece) {
        let oldPiece = this.localChessPieces.get(piece.id);
        while (this.lastMoveHighlightBuffer.length > 0) {
            let removed = this.lastMoveHighlightBuffer.shift();

            if (this.getTileHighlight(removed.x, removed.y) == ColorConstants.TO_HIGHLIGHT_COLOR ||
                this.getTileHighlight(removed.x, removed.y) == ColorConstants.FROM_HIGHLIGHT_COLOR) {
                this.setTileHighlight(removed.x, removed.y, ColorConstants.TRANSPARENT);
            }
        }
        if (oldPiece) {
            let x = oldPiece.x;
            let y = oldPiece.y;

            this.setTileHighlight(x, y, ColorConstants.FROM_HIGHLIGHT_COLOR);
            this.lastMoveHighlightBuffer.push({x: x, y: y});
            this.localChessPieces.set(piece.id, piece.clone());
        }

        let x = piece.x;
        let y = piece.y;
        this.setTileHighlight(x, y, ColorConstants.TO_HIGHLIGHT_COLOR);
        this.lastMoveHighlightBuffer.push({x: x, y: y})
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
                    tileWidth={this.tileWidth}
                    tileHeight={this.tileHeight}
                    board={this.props.gameState.board}/>

                <ChessPieces
                    ref={this.chessPieces}
                    tileWidth={this.tileWidth}
                    tileHeight={this.tileHeight}
                    inverted={this.props.inverted}
                    playerNum={this.props.playerNum}
                    board={this.props.gameState.board}
                    pieces={this.props.gameState.pieces}
                    onRequestMove={this.props.onRequestMove}
                />
            </div>
        );
    }
}

