import {Board} from "./Board";
import {PieceBuilder} from "./PieceBuilder";
import {GameState} from "./GameState";
import {Movement} from "./Movement";
import {BoardLocation} from "./BoardLocation";

export class GameMaster {
    gameState: GameState;

    constructor() {
        let board = new Board();
        let pieces = PieceBuilder.classicSet();
        this.gameState = new GameState(pieces, board);
    }

    makeMove(x0: number, y0: number, xf: number, yf: number) {
        try {
            let startLoc = new BoardLocation(x0, y0);
            let endLoc = new BoardLocation(xf, yf);
            let movement = new Movement(startLoc, endLoc);
            return this.gameState.takeTurn(movement);
        } catch (e) {
            return false;
        }
    }

    toAscii() {
        return this.gameState.toAscii();
    }
}