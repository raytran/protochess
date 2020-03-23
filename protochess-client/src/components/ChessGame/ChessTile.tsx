import React from "react";

interface IProps {
    x: number,
    y: number,
    width: number,
    height: number,
    color: string,
    highlightColor: string
}

export class ChessTile extends React.PureComponent<IProps> {
    render() {
        return (
            <>
                <rect x={this.props.x} y={this.props.y} width={this.props.width} height={this.props.height}
                      style={{fill: this.props.color}}/>
                <rect x={this.props.x} y={this.props.y} width={this.props.width} height={this.props.height}
                      style={{fill: this.props.highlightColor}}/>
            </>
        );
    }
}
