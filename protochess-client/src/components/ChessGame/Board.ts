import {fabric} from "fabric";
import {GameState} from "protochess-shared";
const Board = {
    tileMap: new Map<string,fabric.Rect>(),
    canvas: null,
    tileWidth: -1,
    tileHeight: -1,
    init(canvas:fabric.Canvas,tileWidth:number,tileHeight:number){
        //@ts-ignore
        this.canvas = canvas;
        this.tileWidth = tileWidth;
        this.tileHeight = tileHeight;
    },
    loadBoard(inverted:boolean, gameState:GameState){
        for (let tile of gameState.board.tiles!){
            let x = tile.location.x;
            let y = tile.location.y;
            if (!inverted){
                y = gameState.board.height - y - 1;
            }
            let rect = new fabric.Rect({
                left : x*this.tileWidth,
                top : y*this.tileHeight,
                width : this.tileWidth,
                height : this.tileHeight,
                fill : tile.tileTypeStr == 'b'? '#3c74c8' : '#9dfcff',
                selectable:false,
                evented:false,
            });


            this.tileMap.set("X"+x+"Y"+y,rect);

            //@ts-ignore
            this.canvas.add(rect);
            rect.moveTo(-9999);
        }

        return this.tileMap;
    },

    deleteAll() {
        for (let key of this.tileMap.keys()) {
            let value = this.tileMap.get(key);
            //@ts-ignore
            this.canvas.remove(value!);
            this.tileMap.delete(key);
        }
    }
};

export default Board;