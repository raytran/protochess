import React, {Component} from "react";
import {Redirect} from "react-router";
import GenerateChallengeURL from '../containers/GenerateChallengeURLContainer';
import ClientHandler from "../ClientHandler";


interface OwnState {
    redirect: string | null;
    onGameReady: Function;

}

class Home extends Component<{}, OwnState> {
    constructor(props: {}) {
        super(props);
        let this_ = this;

        function onGameReady() {
            this_.setState({'redirect': "/roomId/" + ClientHandler.getRoomId()});
            ClientHandler.removeGameReadyListener(onGameReady);
        }

        this.state = {'redirect': null, onGameReady: onGameReady};
        ClientHandler.addGameReadyListener(onGameReady);
    }

    componentWillUnmount(): void {
        ClientHandler.removeGameReadyListener(this.state.onGameReady);
    }


    render() {
        if (this.state.redirect) {
            return <Redirect to={this.state.redirect}/>
        }
        return (
            <div>
                <h2>Challenge a friend</h2>
                <GenerateChallengeURL/>
            </div>
        );
    }
}

export default Home;
