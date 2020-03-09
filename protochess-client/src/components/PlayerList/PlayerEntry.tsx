import React, {Component} from "react";
import Button from 'react-bootstrap/Button';
import {ListGroupItem} from "react-bootstrap";

interface MyProps {
    playerId:string;
    playerName:string;
    isLeader:boolean;
    showMakeLeaderButton:boolean;
    handleOnClick:(otherId:string)=>any;
}

export default class PlayerEntry extends Component<MyProps> {
    constructor(props: MyProps) {
        super(props);
        this.onClick = this.onClick.bind(this);
    }
    private onClick(){
        this.props.handleOnClick(this.props.playerId);
    }

    render() {
        return(
            <ListGroupItem>
                {this.props.showMakeLeaderButton ? <Button onClick={this.onClick} variant="secondary" size="sm">Make Leader</Button>:""}
                {this.props.isLeader ? <b> Leader: </b> : " "}
                {this.props.playerName}
            </ListGroupItem>
        )
    }
}

