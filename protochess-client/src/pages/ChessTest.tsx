import React, {Component} from "react";
import {fabric} from "fabric";

class ChessTest extends Component {
    componentDidMount(): void {
        let canvas = new fabric.Canvas('myCanvas');
        canvas.setBackgroundColor('rgba(255, 255, 255, 1)', canvas.renderAll.bind(canvas));
        let rect = new fabric.Rect({
            top : 100,
            left : 100,
            width : 60,
            height : 70,
            fill : 'red'
        });

        canvas.add(rect);
    }

    render() {
        return (
            <canvas id="myCanvas"/>
        );
    }
}

export default ChessTest;
