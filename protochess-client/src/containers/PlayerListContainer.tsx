import React, {Component} from "react";
import ClientHandler from '../ClientHandler';
import {PlayerList} from '../components/PlayerList/PlayerList';
import {Player} from 'protochess-shared';
import {Client} from "colyseus.js";

interface myState{
    connectedUsers: Player[] | null;
    showMakeLeaderButton:boolean;
}
class PlayerListContainer extends Component<{},myState> {
    constructor(props: {}) {
        super(props);

        this.state = {showMakeLeaderButton:false, connectedUsers:null};
        this.onPlayerChange = this.onPlayerChange.bind(this);
    }

    private onPlayerChange() {
        let players = ClientHandler.getPlayers();
        let isLeader = ClientHandler.getIsLeader();
        this.setState({showMakeLeaderButton:isLeader, connectedUsers: players})
    }
    private handleClick(newLeaderId:string):void{
        console.log(newLeaderId);
        ClientHandler.switchLeader(newLeaderId);
    }

    componentDidMount(): void {
        ClientHandler.addPlayerChangeListener(this.onPlayerChange);
    }

    componentWillUnmount(): void {
        ClientHandler.removePlayerChangeListener(this.onPlayerChange)
    }

    render() {
        return (
            <PlayerList className = "playerList"
                        showMakeLeaderButton={this.state.showMakeLeaderButton}
                        players = {this.state.connectedUsers}
                        handleLeaderButtonClick={this.handleClick}
            />
        );
    }
}

export default PlayerListContainer;
