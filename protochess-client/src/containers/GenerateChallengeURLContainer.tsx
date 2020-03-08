import React, {Component} from "react";
import ClientHandler from '../ClientHandler'
import GenerateChallengeURL from '../components/GenerateChallengeURL/GenerateChallengeURL';

interface OwnState {
    showResults: boolean;
    roomId: string;
}

class GenerateChallengeURLContainer extends Component<{}, OwnState> {
    constructor(props: {}) {
        super(props);
        this.state = {roomId: "ERROR: Check server", showResults: false};
        this.onClick = this.onClick.bind(this);
    }

    createRoom() {
        ClientHandler.createPrivateRoom().then(room => {
            this.setState({roomId: room.id});
        }).catch(e => {
            this.setState({roomId: "ERROR: Check server"});
        });
    }

    onClick() {
        this.setState({showResults: true});
        ClientHandler.leaveRoom();
        this.createRoom();
    }


    render() {
        return <GenerateChallengeURL showResults={this.state.showResults} onClickHandler={this.onClick}
                                     displayText={window.location.origin + "/roomId/" + this.state.roomId}/>;
    }
}

export default GenerateChallengeURLContainer;
