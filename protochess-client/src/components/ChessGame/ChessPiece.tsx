import React from "react";

//@ts-ignore

interface IProps {
    pieceId: string
    backgroundImage: string,
    x: number,
    y: number,
    height: number,
    width: number
}

export class ChessPiece extends React.PureComponent<IProps> {
    constructor(props: IProps) {
        super(props);
    }

    render() {
        return (
            <div
                key={this.props.pieceId}
                id={this.props.pieceId}
                style={{
                    display: 'flex',
                    textAlign: 'center',
                    justifyContent: 'center',
                    alignItems: 'center',
                    width: this.props.width,
                    height: this.props.height,
                    position: "absolute",
                    left: this.props.x,
                    top: this.props.y,
                    transition: 'top 300ms ease-in-out, left 300ms ease-in-out',
                    backgroundImage: this.props.backgroundImage,
                    backgroundSize: 'contain'
                }}
            >
                {}
            </div>
        );
    }
}
