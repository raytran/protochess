import React, {Component} from "react";
import ClientHandler from '../ClientHandler';
import ChatContainer from '../containers/ChatContainer';
import PlayerListContainer from "../containers/PlayerListContainer";
import ChessGameContainer from "../containers/ChessGameContainer";
import {Player} from 'protochess-shared';
import './Lobby.css';
import {Button} from "react-bootstrap";

interface myState {
    displayText: string;
    name: string;
    connectedUsers: Player[] | null;
    showStartGameButton: boolean;
    showGame: boolean | null;
}

class Lobby extends Component<{}, myState> {
    constructor(props: {}) {
        super(props);
        let roomId = "";
        try {
            roomId = "" + window.location.href.match(/roomId\/([a-zA-Z0-9\-_]+)/)![1];
            console.log(roomId);
            this.state = {
                showGame: null,
                showStartGameButton: false,
                displayText: roomId,
                name: "",
                connectedUsers: null
            };
        } catch (err) {
            this.state = {
                showGame: null,
                showStartGameButton: false,
                displayText: "Url error",
                name: "",
                connectedUsers: null
            };
        }
        this.onPlayerChange = this.onPlayerChange.bind(this);
        this.onStartGame = this.onStartGame.bind(this);
    }

    componentDidMount(): void {
        if (!ClientHandler.isConnected()) {
            ClientHandler.joinById(this.state.displayText).then(newRoom => {
                console.log("joined successfully", newRoom);
            }).catch(e => {
                console.error("join error", e);
                this.setState({"displayText": "No such room"});
            });
        }

        ClientHandler.addPlayerChangeListener(this.onPlayerChange);
        ClientHandler.addRedirectListener(this.onStartGame);
    }

    componentWillUnmount(): void {
        ClientHandler.removeRedirectListener(this.onStartGame);
        ClientHandler.removePlayerChangeListener(this.onPlayerChange)
    }

    render() {
        if (this.state.showGame) {
            return (
                <div id={'content'}>
                    <div style={{flexShrink: 0, flexGrow: 0}} id={'chessGame'}>
                        <ChessGameContainer width={700} height={450}/>
                    </div>
                    <div id={"auxillary"}>
                        <PlayerListContainer/>
                        <ChatContainer/>
                    </div>
                </div>
            );
        } else return (
            <div>
                <h2>Lobby — {this.state.displayText} — {this.state.name} </h2>
                <h4>Connected Users: </h4>
                <div className='rowC'>
                    <PlayerListContainer/>
                    <ChatContainer/>
                    {this.state.showStartGameButton ?
                        <Button id={'startGameButton'} variant="warning" onClick={this.onStartGameButtonClick}>Start
                            Game</Button> : ""}
                </div>
            </div>
        );
    }

    private onPlayerChange(): void {

        let isLeader = ClientHandler.getIsLeader();
        this.setState({showStartGameButton: isLeader, name: ClientHandler.getName()});
    }

    private onStartGameButtonClick() {
        ClientHandler.startGame();
    }

    private onStartGame(type: string) {
        if (type === "startGame") {
            this.setState({showGame: true});
        }
    }
}

export default Lobby;
