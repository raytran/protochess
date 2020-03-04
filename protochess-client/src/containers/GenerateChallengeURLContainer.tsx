import React, {Component} from "react";
import client from '../client'
import GenerateChallengeURL from '../components/GenerateChallengeURL/GenerateChallengeURL';
import * as Colyseus from "colyseus.js";
interface OwnState {
    showResults: boolean;
    roomId: string;
    room: Colyseus.Room | null;
}

class GenerateChallengeURLContainer extends Component<{}, OwnState> {
    constructor(props: {}) {
        super(props);
        this.state = {room: null, roomId: "ERROR: Check server", showResults: false};
        this.onClick = this.onClick.bind(this);
    }

    createRoom(){
        client.create("my_room", {'private': true}).then((room: any): any => {
            console.log("joined successfully", room);
            this.setState({room: room, roomId: room.id});
        }).catch(e => {
            console.error("join error", e);
        });
    }


    onClick() {
        if (!this.state.showResults){
            this.setState({showResults: true});
            this.createRoom();
        }else{
            //Remove ourselves from the existing room
            //Generate a new room
            if (this.state.room != null) this.state.room.leave();
            this.createRoom();
        }

    }

    render() {
        return <GenerateChallengeURL showResults={this.state.showResults} onClickHandler={this.onClick}
                                     displayText={window.location.origin +"/roomId/" + this.state.roomId}/>;
    }
}

export default GenerateChallengeURLContainer;
