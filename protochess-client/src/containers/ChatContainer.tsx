import React, {Component} from "react";
import ClientHandler from '../ClientHandler';
import {ChatBox} from '../components/ChatBox/ChatBox';

interface myState {
    chatText: string;
}

class ChatContainer extends Component<{}, myState> {
    constructor(props: {}) {
        super(props);
        this.state = {chatText: ""};
        this.handleInput = this.handleInput.bind(this);
        this.onChat = this.onChat.bind(this);

    }

    componentDidMount(): void {
        ClientHandler.addChatListener(this.onChat);
    }

    componentWillUnmount(): void {
        ClientHandler.removeChatListener(this.onChat);
    }

    render() {
        return (
            <ChatBox className="chat" displayText={this.state.chatText} handleOnInput={this.handleInput}/>
        );
    }

    private onChat(data: any) {
        let newMsg = data['senderName'] + ": " + data['chatMsg'];
        this.setState({chatText: this.state.chatText + "\n" + newMsg});
    }

    private handleInput(input: string) {
        ClientHandler.sendMessage({chatMsg: input});
    }
}

export default ChatContainer;
