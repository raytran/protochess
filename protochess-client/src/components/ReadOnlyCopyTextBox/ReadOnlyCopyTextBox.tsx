import React, {Component} from "react";
import {Button, FormControl, InputGroup} from 'react-bootstrap';
import CopyToClipboard from 'react-copy-to-clipboard';
import './ReadOnlyCopyTextBox.css'

interface MyProps {
    value: string;
}

class ReadOnlyCopyTextBox extends Component<MyProps> {
    constructor(props: MyProps) {
        super(props);
    }

    render() {
        return (
            <div>
                <InputGroup className="mb-3">
                    <FormControl
                        readOnly
                        className="input-disabled"
                        value={this.props.value}
                        aria-describedby="basic-addon2"
                    />
                </InputGroup>
                <CopyToClipboard text={this.props.value}
                                 onCopy={() => this.setState({})}>
                    <Button variant="secondary">Copy to clipboard</Button>
                </CopyToClipboard>
            </div>
        )
    }
}

export default ReadOnlyCopyTextBox;
