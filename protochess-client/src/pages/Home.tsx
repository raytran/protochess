import React, {Component} from "react";
import GenerateChallengeURL from '../containers/GenerateChallengeURLContainer';

class Home extends Component {
    render() {
        return (
            <div>
                <h2>Challenge a friend</h2>
                <GenerateChallengeURL/>
            </div>
        );
    }
}

export default Home;
