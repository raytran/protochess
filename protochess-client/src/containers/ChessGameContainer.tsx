import React, {Component, Ref} from "react";
import * as Colyseus from "colyseus.js";
import {Piece} from "protochess-shared";
import ClientHandler from '../ClientHandler';
import ChessGame from '../components/ChessGame/ChessGame';
import {ColorConstants} from "../components/ChessGame/ColorConstants";

interface IProps{
    width:number,
    height:number
}
interface IState{
    inverted:boolean | null,
    playerNum:number | null
}
export default class ChessGameContainer extends Component<IProps,IState> {
    private chessGame:Ref<ChessGame>;
    constructor(props:IProps) {
        super(props);
        this.chessGame = React.createRef();
        this.onPieceChange = this.onPieceChange.bind(this);
        this.onPieceDelete = this.onPieceDelete.bind(this);
        this.onPlayerNum = this.onPlayerNum.bind(this);
        this.onPlayerChange = this.onPlayerChange.bind(this);
        this.onGameStateChange = this.onGameStateChange.bind(this);
        this.state = {inverted:null,playerNum:null};
    }
    private setKingTileColor(playerNum:number,color:string) {
        for (let id in ClientHandler.getRoom()!.state.gameState.pieces) {
            const piece: Piece = ClientHandler.getRoom()!.state.gameState.pieces[id];
            if (piece.pieceTypeStr == 'k' || piece.pieceTypeStr == "K") {
                //@ts-ignore
                if (piece.owner == playerNum) {
                    console.log(piece);
                    //@ts-ignore
                    this.chessGame.current.setTileHighlight(piece.location.x, piece.location.y, color);
                }
            }
        }
    }
    private onPlayerChange(changes:Colyseus.DataChange[]){
        if (changes){
            //@ts-ignore
            if (changes['inCheck']){
                console.log("IN CHECK!!");
                //@ts-ignore
                this.setKingTileColor(changes['playerNum'],ColorConstants.IN_CHECK_HIGHLIGHT_COLOR);
            }else{
                //@ts-ignore
                this.setKingTileColor(changes['playerNum'],'rgba(0,0,0,0)');
            }
        }
    }

    private onPlayerNum(){
        let playerNum = ClientHandler.getPlayerNum();
        if (playerNum !== null && playerNum >= 0){
            if (playerNum! % 2 == 1){
                this.setState({inverted:true,playerNum:playerNum})
            }else{
                this.setState({inverted:false,playerNum:playerNum})
            }
        }
    }
    private onGameStateChange(changes:Colyseus.DataChange[]){
        changes.forEach(change => {
            //@ts-ignore
            if (change.field == 'whosTurn'){
                //@ts-ignore
                this.chessGame.current.lockAllPieces();
                //@ts-ignore
                if (this.state.playerNum === change.value) {
                    //@ts-ignore
                    this.chessGame.current.unlockPieces(change.value);
                }
            }
            //@ts-ignore
            else if (change.field == 'winner'){
                let winner = change.value;
                //@ts-ignore
                this.chessGame.current.displayWinner(winner);


            }
        });

    }
    private onPieceChange(piece:Piece){
        if (this.chessGame){
            //@ts-ignore
            this.chessGame.current.updatePiece(piece);
        }
    }

    private onPieceDelete(piece:Piece){
        if (this.chessGame){
            //@ts-ignore
            this.chessGame.current.deletePiece(piece);
        }
    }

    private onRequestMove(move:{id:string,x:number,y:number}){
        ClientHandler.requestMove(move);

    }
    componentDidMount(): void {
        ClientHandler.addPlayerNumListener(this.onPlayerNum);
        ClientHandler.addGameStateListener(this.onGameStateChange);
        ClientHandler.addPlayerChangeListener(this.onPlayerChange);
        ClientHandler.addPieceChangeListener(this.onPieceChange);
        ClientHandler.addPieceDeleteListener(this.onPieceDelete);
    }
    componentWillUnmount(): void {
        ClientHandler.removePieceChangeListener(this.onPieceChange);
        ClientHandler.removePieceDeleteListener(this.onPieceDelete);
        ClientHandler.removePlayerNumListener(this.onPlayerNum);
        ClientHandler.removePlayerChangeListener(this.onPlayerChange);
        ClientHandler.removeGameStateListener(this.onGameStateChange);
    }

    render() {
        return (
            ClientHandler.getRoom() ? <ChessGame width = {this.props.width} height = {this.props.height} ref={this.chessGame} playerNum={this.state.playerNum} inverted={this.state.inverted} requestMove={this.onRequestMove} gameState={ClientHandler.getRoom()!.state.gameState}/> : ""
        );
    }
}

