import React, {Component} from 'react';
import './App.css';
import 'bootstrap/dist/css/bootstrap.min.css';
import {
  Route,
  NavLink,
  Switch,
  HashRouter as Router
} from "react-router-dom";
import Home from "./Home";
import Stuff from "./Stuff";
import Contact from "./Contact";
import OpponentConnector from "./OpponentConnector";
class App extends Component {
 render() {
    return (
     <Router>
        <div>
          <h1>Protochess</h1>
          <ul className="header">
            <li><NavLink exact to="/">Home</NavLink></li>
            <li><NavLink to="/stuff">Stuff</NavLink></li>
            <li><NavLink to="/contact">Contact</NavLink></li>
          </ul>
          <div className="content">
              <Switch>
                <Route exact path="/" component={Home}/>
                <Route path="/stuff" component={Stuff}/>
                <Route path="/contact" component={Contact}/>
                <Route path="/roomId/" component={OpponentConnector}/>
              </Switch>
          </div>
        </div>

      </Router>
    );
  }
}

export default App;
