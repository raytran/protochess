import {MapSchema, Schema, type} from "@colyseus/schema";
import {Piece} from "./Piece";
import {Board} from "./Board";

export class GameState extends Schema {
    @type("number")
    whosTurn: number; //who's turn is it? --0 for white 1 for black 2.3..etc
    @type("number")
    winner: number = -1;
    @type("number")
    numPlayers: number;
    @type("boolean")
    useChecks: boolean;
    @type("boolean")
    useCheckmates: boolean;
    @type(Board)
    board: Board;
    @type({map: Piece})
    pieces: MapSchema<Piece>;

    constructor() {
        super();
        this.useChecks = true;
        this.useCheckmates = true;
        this.pieces = new MapSchema<Piece>();
        this.board = new Board();
        this.whosTurn = 0;
        this.numPlayers = 2;
    }
}

