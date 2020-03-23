import React, {Component} from "react";
import {Board} from "protochess-shared";
import {ChessTile} from "./ChessTile";
import {ColorConstants} from "./ColorConstants";
import {Map as IMap} from "immutable";

interface IProps {
    inverted: boolean,
    tileWidth: number,
    tileHeight: number,
    board: Board
}

interface IState {
    tileColorMap: IMap<string, string>,
    tileHighlightMap: IMap<string, string>
}

export default class ChessBoard extends Component<IProps, IState> {
    constructor(props: IProps) {
        super(props);
        let tileColorMap = new Map<string, string>();
        let tileHighlightMap = new Map<string, string>();
        for (let tile of this.props.board.tiles!) {
            let color = tile.tileTypeStr == 'b' ? ColorConstants.DARK_SQUARE : ColorConstants.LIGHT_SQUARE;
            tileColorMap.set("x" + tile.location.x + "y" + tile.location.y, color);
            tileHighlightMap.set("x" + tile.location.x + "y" + tile.location.y, ColorConstants.TRANSPARENT);
        }

        //@ts-ignore
        this.state = {tileColorMap: new IMap(tileColorMap), tileHighlightMap: new IMap(tileHighlightMap)}
    }

    setTileHighlight(x: number, y: number, color: string) {
        this.setState({tileHighlightMap: this.state.tileHighlightMap.set("x" + x + "y" + y, color)})
    }

    getTileHighlight(x: number, y: number) {
        return this.state.tileHighlightMap.get("x" + x + "y" + y);
    }

    render() {
        return (
            <svg style={{width: '100%', height: '100%'}}>
                {this.props.board.tiles!.map((tile) => (
                    <ChessTile key={tile.location.x + "" + tile.location.y}
                               x={tile.location.x * this.props.tileWidth}
                               y={this.props.inverted ? tile.location.y * this.props.tileHeight : (this.props.board.height - tile.location.y - 1) * this.props.tileHeight}
                               width={this.props.tileWidth}
                               height={this.props.tileHeight}
                               highlightColor={this.state.tileHighlightMap.get("x" + tile.location.x + "y" + tile.location.y)!}
                               color={this.state.tileColorMap.get("x" + tile.location.x + "y" + tile.location.y)!}/>
                ))}

            </svg>
        );
    }
}

