//Holds Tiles
import {Tile} from "./Tile";
import {TileType} from "./TileType";
import {BoardLocation} from "./BoardLocation";

//Represents just the tiles NOT pieces
export class Board{
    tiles:Tile[][] | null;
    width:number;
    height:number;
    constructor() {
        this.width = 8;
        this.height = 8;
        this.tiles = [];
    }
    //Build the board to a classic board
    private buildClassicBoard():void{
        this.width = 8;
        this.height = 8;
        this.tiles = [];

        for (let i=0;i<this.height;i++){
            this.tiles.push([]);
            for (let j=0;j<this.width;j++){
                if ((i+j) % 2 ==0){
                    let tile = new Tile(new BoardLocation(j,i),TileType.Black);
                    this.tiles[i].push(tile);
                }else{
                    let tile = new Tile(new BoardLocation(j,i),TileType.White);
                    this.tiles[i].push(tile);
                }
            }
        }
    }

    onBoard(boardLocation:BoardLocation){
        if (this.width == null || this.height == null){
            throw new Error("Board is not built");
        }
        return (boardLocation.x >= 0
            && boardLocation.y >= 0
            && boardLocation.x < this.width
            && boardLocation.y < this.height);
    }

    //Prints the board tiles
    toAscii():string{
        if (this.width == null || this.height == null){
            throw new Error("Board is not built");
        }
        let toReturn = "";
        for (let i=this.height-1;i>=0;i--){
            for (let j=0;j<this.width;j++){
                toReturn += this.tiles![i][j].toAscii();
            }
            toReturn += i+'\n';
        }
        return toReturn;
    }
    //Instantiate
    build(){
        this.buildClassicBoard();
    }

}