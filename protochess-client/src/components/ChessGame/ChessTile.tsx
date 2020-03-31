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
                <div style={{
                    position: "absolute",
                    width: this.props.width,
                    height: this.props.height,
                    left: this.props.x,
                    top: this.props.y,
                    backgroundColor: this.props.color
                }}/>
                <div style={{
                    position: "absolute",
                    left: this.props.x,
                    top: this.props.y,
                    width: this.props.width,
                    height: this.props.height,
                    backgroundColor: this.props.highlightColor
                }}/>
            </>
        );
    }
}
