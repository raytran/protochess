import React, { Component } from "react";
import client from './client';
 
class OpponentConnector extends Component<{},{displayText:string}> {
 constructor(props:{}){
    super(props);

    let roomId = window.location.href.match(/roomId\/([a-zA-Z0-9\-_]+)/)![1];
    console.log(roomId);
    this.state = {"displayText":roomId}

    client.joinById(roomId, {/* options */}).then(room => {
      console.log("joined successfully", room);
    }).catch(e => {
      console.error("join error", e);
      this.setState({"displayText":"No such room"});
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
