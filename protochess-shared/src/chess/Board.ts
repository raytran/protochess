//Holds Tiles
import {Tile} from "./Tile";
import {TileType} from "./TileType";
import {BoardLocation} from "./BoardLocation";
import {ArraySchema, Schema, type} from "@colyseus/schema";

//Represents just the tiles NOT pieces
export class Board extends Schema {
    @type([Tile])
    tiles: ArraySchema<Tile> | null;
    @type('number')
    width: number;
    @type('number')
    height: number;

    constructor() {
        super();
        this.width = 8;
        this.height = 8;
        this.tiles = new ArraySchema<Tile>();
        this.buildClassicBoard();
    }

    onBoard(boardLocation: BoardLocation) {
        if (this.width == null || this.height == null) {
            throw new Error("Board is not built");
        }
        return (boardLocation.x >= 0
            && boardLocation.y >= 0
            && boardLocation.x < this.width
            && boardLocation.y < this.height);
    }

    //Prints the board tiles
    toAscii(): string {
        if (this.width == null || this.height == null) {
            throw new Error("Board is not built");
        }
        let toReturn = "";
        for (let i = this.height - 1; i >= 0; i--) {
            for (let j = 0; j < this.width; j++) {
                toReturn += this.getTile(j, i)!.toAscii();
            }
            toReturn += i + '\n';
        }
        return toReturn;
    }

    getTile(x: number, y: number) {
        if (this.tiles != null)
            return this.tiles[y * this.width + x];
    }

    //Instantiate
    build() {
        this.buildClassicBoard();
    }

    //Build the board to a classic board
    private buildClassicBoard(): void {
        this.width = 8;
        this.height = 8;
        this.tiles = new ArraySchema<Tile>();

        for (let i = 0; i < this.height; i++) {
            for (let j = 0; j < this.width; j++) {
                if ((i + j) % 2 == 0) {
                    let tile = new Tile(new BoardLocation(j, i), TileType.Black);
                    this.tiles.push(tile);
                } else {
                    let tile = new Tile(new BoardLocation(j, i), TileType.White);
                    this.tiles.push(tile);
                }
            }
        }
    }

}