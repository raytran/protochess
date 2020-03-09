import React, {Component} from "react";
import ClientHandler from '../ClientHandler';
import ChatContainer from '../containers/ChatContainer';
import PlayerListContainer from "../containers/PlayerListContainer";
import {Player} from 'protochess-shared';
import './Lobby.css';

interface myState{
    displayText: string;
    name:string;
    connectedUsers: Player[] | null;
}
class Lobby extends Component<{},myState> {
    constructor(props: {}) {
        super(props);
        let roomId = "";
        try {
            roomId = "" + window.location.href.match(/roomId\/([a-zA-Z0-9\-_]+)/)![1];
            console.log(roomId);
            this.state = {displayText: roomId, name: "", connectedUsers: null };
        } catch (err) {
            this.state = {displayText: "Url error", name: "" ,connectedUsers: null};
        }
        this.onPlayerChange = this.onPlayerChange.bind(this);
    }
    
    private onPlayerChange():void{
        this.setState({name:ClientHandler.getName()});
    }

    componentDidMount(): void {
        if (!ClientHandler.isConnected()) {
            ClientHandler.joinById(this.state.displayText).then(newRoom => {
                console.log("joined successfully", newRoom);
                console.log("YO WE IN hERE");
            }).catch(e => {
                console.error("join error", e);
                this.setState({"displayText": "No such room"});
            });
        }

        ClientHandler.addPlayerChangeListener(this.onPlayerChange);
    }

    componentWillUnmount(): void {
        ClientHandler.removePlayerChangeListener(this.onPlayerChange)
    }

    render() {
        return (
            <div>
                <h2>Lobby — {this.state.displayText} — {this.state.name} </h2>
                <h4>Connected Users: </h4>
                <div className='rowC'>
                    <PlayerListContainer/>
                    <ChatContainer/>
                </div>
            </div>
        );
    }
}

export default Lobby;
