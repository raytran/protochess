import React, { Component } from "react";
import Button from 'react-bootstrap/Button';
import ReadOnlyCopyTextBox from '../components/ReadOnlyCopyTextBox';
import './GenerateRoomLink.css'
import client from '../client'
import { Room, Client } from "colyseus";

interface OwnState {
    showResults: boolean;
    roomId:string;
}
class GenerateRoomLink extends Component<{},OwnState> {
  constructor(props:{}){
       super(props)
       this.state = { roomId:"None", showResults: false };
       this.onClick = this.onClick.bind(this);
  }

  onClick() {
       this.setState({ showResults: true });
       client.create("my_room", {'private': true}).then((room:any):any => {
          console.log("joined successfully", room);
          this.setState({ roomId: room.id });
        }).catch(e => {
          console.error("join error", e);
        });

  }

  render() {
    return(
       <div className = "genRoomLink">
       <Button className = "genUrlButton" onClick = {this.onClick} variant="light" >
       Generate URL
       </Button>
       <div className = "copyText"> 
       {this.state.showResults ? <ReadOnlyCopyTextBox value={this.state.roomId}/> : null}
       </div>
       </div>
    )
  }
}
 
export default GenerateRoomLink;
