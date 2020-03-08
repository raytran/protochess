import React, {Component} from "react";
import ClientHandler from '../ClientHandler';

class OpponentConnector extends Component<{}, { displayText: string }> {
    constructor(props: {}) {
        super(props);
        let roomId = "";
        try {
            roomId = "" + window.location.href.match(/roomId\/([a-zA-Z0-9\-_]+)/)![1];
            console.log(roomId);
            this.state = {"displayText": roomId};
        } catch (err) {
            this.state = {"displayText": "Room error"};
        }


        ClientHandler.joinById(roomId).then(newRoom => {
            console.log("joined successfully", newRoom);
        }).catch(e => {
            console.error("join error", e);
            this.setState({"displayText": "No such room"});
        });

    }

    render() {
        return (
            <div>
                <h2>OpponentConnector</h2>
                <h3>{this.state.displayText}</h3>
            </div>
        );
    }
}

export default OpponentConnector;
