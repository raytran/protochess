import {fabric} from "fabric";
import {GameState} from "protochess-shared";
import {ColorConstants} from "./ColorConstants";
const Board = {
    tileMap: new Map<string,any>(),
    canvas: null,
    gameWidth:0,
    gameHeight:0,
    tileWidth: -1,
    tileHeight: -1,
    highlightedSquares:new Array<any>(),
    init(canvas:fabric.Canvas,tileWidth:number,tileHeight:number){
        //@ts-ignore
        this.canvas = canvas;
        this.tileWidth = tileWidth;
        this.tileHeight = tileHeight;
    },
    loadBoard(inverted:boolean,gameState:GameState){
        for (let tile of gameState.board.tiles!){
            let x = tile.location.x;
            let y = tile.location.y;
            this.gameWidth = gameState.board.width;
            this.gameHeight = gameState.board.height;
            if (!inverted){
                y = gameState.board.height - y - 1;
            }
            let rect = new fabric.Rect({
                left : x*this.tileWidth,
                top : y*this.tileHeight,
                width : this.tileWidth,
                height : this.tileHeight,
                fill : tile.tileTypeStr == 'b'? ColorConstants.DARK_SQUARE : ColorConstants.LIGHT_SQUARE
            });
            let highLightRect = fabric.util.object.clone(rect);
            highLightRect.set('fill',ColorConstants.TRANSPARENT);
            highLightRect.set('opacity',0.5);


            let group = new fabric.Group([rect,highLightRect]);
            group.selectable = false;
            group.evented = false;

            this.tileMap.set("X"+x+"Y"+y,{tile:tile,highlight:highLightRect,rect:rect,fabric:group});

            //@ts-ignore
            this.canvas.add(group);
            rect.moveTo(-9999);
        }

        return this.tileMap;
    },

    fabricPosToXY(inverted:boolean,left:number ,top:number){
        let x,y;
        x = left / this.tileWidth;
        y = top / this.tileHeight;
        if (!inverted){
            y = this.gameHeight - y - 1;
        }
        return {x:x,y:y}
    },
    updateBoardHighlight(inverted:boolean,x:number,y:number,color:string){
        while (this.highlightedSquares.length >= 2){
            let removed = this.highlightedSquares.shift();
            console.log(removed.highlight.fill);
            if (removed.highlight.fill == ColorConstants.LAST_MOVE_HIGHLIGHT_COLOR){
                removed.highlight.set('fill',ColorConstants.TRANSPARENT)
            }
            //@ts-ignore
            this.canvas.renderAll();
        }
        if (!inverted){
            y = this.gameHeight - y - 1;
        }
        let obj = this.tileMap.get("X"+x+"Y"+y);
        obj.highlight!.set('fill',color);
        this.highlightedSquares.push(obj!);
        //@ts-ignore
        this.canvas.renderAll();
    },
    setHighlightColor(inverted:boolean,x:number,y:number,color:string){
        if (!inverted){
            y = this.gameHeight - y - 1;
        }
        console.log("we here bruhs");
        let obj = this.tileMap.get("X"+x+"Y"+y);
        if (obj) {
            obj.highlight!.set('fill', color);
        }
        //@ts-ignore
        this.canvas.renderAll();
    },

    getFabricObj(x:number,y:number){
        return this.tileMap.get("X"+x+"Y"+y).fabric;
    },

    deleteAll() {
        for (let key of this.tileMap.keys()) {
            let value = this.tileMap.get(key).fabric;
            //@ts-ignore
            this.canvas.remove(value!);
            this.tileMap.delete(key);
        }
    }
};

export default Board;