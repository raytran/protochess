import React, { Component } from "react";
import GenerateRoomLink from './containers/GenerateRoomLink';
 
class Home extends Component {
  render() {
    return (
      <div>
        <h2>Challenge a friend</h2>
        <GenerateRoomLink/>
      </div>
    );
  }
}
 
export default Home;
