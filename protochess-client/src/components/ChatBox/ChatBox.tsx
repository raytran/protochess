import React, {Component} from "react";
import {Form,FormControl} from 'react-bootstrap';
import './ChatBox.css';

interface Props {
    handleOnInput:Function;
    displayText:string;
    className:string | null;

}
export class ChatBox extends Component<Props, {}> {
    private textInput = React.createRef<HTMLInputElement>();
    private displayBox = React.createRef<HTMLTextAreaElement>();

    constructor(props:Props) {
        super(props);
        this.onKeyDown = this.onKeyDown.bind(this);
    }
    readonly onKeyDown = (e: React.KeyboardEvent<HTMLDivElement>) => {
        if (e.key == "Enter"){
            console.log("input!");
            const node = this.textInput.current;
            if (node) {
                this.props.handleOnInput(node.value);
                node.value = "";
            }
        }
    };
    componentDidUpdate(prevProps: Readonly<Props>, prevState: Readonly<{}>, snapshot?: any): void {
        const node = this.displayBox.current;
        if (node){
            node.scrollTop = node.scrollHeight;
        }
    }

    render() {
        return (
            <div className={this.props.className ? this.props.className : ""}>
                <Form.Control id={'chat-textarea'} ref={this.displayBox as React.RefObject<any>} value={this.props.displayText} className="input-disabled" disabled as="textarea" rows="3"/>
                <Form.Control id={'chat-input'} ref={this.textInput as React.RefObject<any>} as="input" onKeyDown={this.onKeyDown} placeholder="Enter text to send"/>
            </div>
        )
    }
}