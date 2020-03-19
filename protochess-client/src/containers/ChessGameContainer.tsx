import React, {Component} from "react";
import ClientHandler from '../ClientHandler';
import ChessGame from '../components/ChessGame/ChessGame';

export default class ChessGameContainer extends Component {
    render() {
        return (
            ClientHandler.getRoom() ? <ChessGame gameState={ClientHandler.getRoom()!.state.gameState}/> : ""
        );
    }
}

