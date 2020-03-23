import React, {Component} from "react";
import {Player} from 'protochess-shared';
import PlayerEntry from './PlayerEntry';
import {ListGroup} from "react-bootstrap";

interface MyProps {
    players: Player[] | null;
    className: string | null;
    showMakeLeaderButton: boolean;
    handleLeaderButtonClick: (arg0: string) => any
}

export class PlayerList extends Component<MyProps> {
    constructor(props: MyProps) {
        super(props);
    }

    render() {
        return (
            <div className={this.props.className ? this.props.className : ""}>
                <ListGroup horizontal>
                    {
                        this.props.players ? this.props.players.map((item: Player, index: number) => (
                            <PlayerEntry key={item.id ? item.id : index}
                                         playerId={item.id}
                                         playerName={item.name}
                                         isLeader={item.isLeader}
                                         handleOnClick={this.props.handleLeaderButtonClick}
                                         showMakeLeaderButton={this.props.showMakeLeaderButton}
                            />
                        )) : <p>None</p>
                    }
                </ListGroup>
            </div>
        )
    }
}

