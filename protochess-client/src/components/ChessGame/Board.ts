import {fabric} from "fabric";
import {GameState} from "protochess-shared";
import {Group} from "fabric/fabric-impl";
const Board = {
    loadBoard(canvas:fabric.Canvas, gameState:GameState,tileWidth:number,tileHeight:number){
        let tileMap = new Map<string,fabric.Rect>();
        for (let tile of gameState.board.tiles!){
            let x = tile.location.x;
            let y = tile.location.y;
            let rect = new fabric.Rect({
                left : x*tileWidth,
                top : y*tileHeight,
                width : tileWidth,
                height : tileHeight,
                fill : tile.tileTypeStr == 'b'? '#3c74c8' : '#9dfcff',
                selectable:false,
                evented:false,
            });


            tileMap.set("X"+x+"Y"+y,rect);

            canvas.add(rect);
            rect.moveTo(-9999);
        }

        return tileMap;
    }
};

export default Board;