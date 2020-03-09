import React, {Component} from "react";
import Button from 'react-bootstrap/Button';
import ReadOnlyCopyTextBox from '../ReadOnlyCopyTextBox/ReadOnlyCopyTextBox';
import './GenerateChallengeURL.css'

interface Props {
    displayText: string;
    showResults: boolean;
    onClickHandler: React.MouseEventHandler<HTMLButtonElement>;
}

class GenerateChallengeURL extends Component<Props, {}> {
    render() {
        return (
            <div className="genRoomLink">
                <Button className="genUrlButton" onClick={this.props.onClickHandler} variant="light">
                    Generate URL
                </Button>
                <div className="copyText">
                    {this.props.showResults ? <ReadOnlyCopyTextBox value={this.props.displayText}/> : null}
                </div>
            </div>
        );
    }
}

export default GenerateChallengeURL;
