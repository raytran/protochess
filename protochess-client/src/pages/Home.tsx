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

        function onGameReady(type:string) {
            if (type === "redirectChallenger") {
                this_.setState({'redirect': "/roomId/" + ClientHandler.getRoomId()});
            }
            ClientHandler.removeRedirectListener(onGameReady);
        }

        this.state = {'redirect': null, onGameReady: onGameReady};
        ClientHandler.addRedirectListener(onGameReady);
    }

    componentWillUnmount(): void {
        ClientHandler.removeRedirectListener(this.state.onGameReady);
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
