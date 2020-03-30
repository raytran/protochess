import {Client, Room} from "colyseus";
import ProtochessEngineJS from "protochess-engine";
import {Piece, Player, State, Tile} from "protochess-shared"

export class StateHandlerRoom extends Room<State> {
    maxClients = 4;
    leaderSessionId: string | null = null;
    didRedirect = false;
    protochessEngine = new ProtochessEngineJS();

    onCreate(options: any) {
        console.log("StateHandlerRoom created!", options);
        let newState = new State();
        let engineState = this.protochessEngine.getState();
        newState.gameState.board.width = engineState.board.width;
        newState.gameState.board.height = engineState.board.height;
        for (let tile of engineState.board.tiles) {
            let localTile = new Tile(tile.x, tile.y, tile.charRep);
            newState.gameState.board.tiles.push(localTile);
        }

        for (let playerNum in engineState.players) {
            let player = engineState.players[playerNum];
            console.log(playerNum);
            for (let pieceID in player) {
                let piece = player[pieceID];
                newState.gameState.pieces[pieceID] = new Piece(parseInt(playerNum), piece.x, piece.y, piece.charRep, pieceID);
            }
        }
        this.setState(newState);
        console.log(JSON.stringify(this.state, null, 2));
    }

    onJoin(client: Client) {
        console.log(client.sessionId + "join");
        if (this.clients.length == 1) {
            this.state.createPlayer(client.sessionId, true, "");
            this.switchLeader(client.sessionId);
        } else {
            this.state.createPlayer(client.sessionId, false, "");
        }
        if (this.clients.length == 2 && !this.didRedirect) {
            this.broadcast({redirectChallenger: true});
            this.didRedirect = true;
        }
    }

    onLeave(client: Client) {
        console.log(client.sessionId + "leave");
        this.state.removePlayer(client.sessionId);
        if (client.sessionId == this.leaderSessionId) {
            for (let id in this.state.players) {
                this.switchLeader(id);
                console.log("Leader updated");
                break;
            }
        }
    }

    onMessage(client: Client, data: any) {
        if (data['takeTurn']) {

            let thisPlayerNum = this.state.players[client.sessionId].playerNum;
            if (this.state.gameState.whosTurn == thisPlayerNum
                && this.state.gameState.pieces[data['move']['id']]) {
                let piece = this.state.gameState.pieces[data['move']['id']];
                let startX = piece.x;
                let startY = piece.y;
                let endX = data['move']['x'];
                let endY = data['move']['y'];
                let result = this.protochessEngine.takeTurn(startX, startY, endX, endY, thisPlayerNum);

                console.log("RESULT:");
                console.log(result);
                if (result.success) {
                    //Update checks
                    for (let id in this.state.players) {
                        const player: Player = this.state.players[id];
                        player.inCheck = false;
                    }

                    for (let playersIdsInCheck of result.playersInCheck) {
                        for (let id in this.state.players) {
                            const player: Player = this.state.players[id];
                            if (player.playerNum == playersIdsInCheck) {
                                player.inCheck = true;
                            }
                        }
                    }
                    //Update turn counter
                    this.state.gameState.whosTurn = result.nextToMove;

                    //Update pieces
                    let enginePieceIDS = [];
                    let engineState = this.protochessEngine.getState();
                    for (let playerNum in engineState.players) {
                        let player = engineState.players[playerNum];
                        for (let pieceID in player) {
                            enginePieceIDS.push(pieceID);
                            let enginePiece = player[pieceID];
                            if (this.state.gameState.pieces[pieceID].x != enginePiece.x
                                || this.state.gameState.pieces[pieceID].y != enginePiece.y
                                || this.state.gameState.pieces[pieceID].pieceTypeStr != enginePiece.charRep) {
                                this.state.gameState.pieces[pieceID].x = enginePiece.x;
                                this.state.gameState.pieces[pieceID].y = enginePiece.y;
                                this.state.gameState.pieces[pieceID].pieceTypeStr = enginePiece.charRep;
                            }
                        }
                    }

                    let toDelete = [];
                    for (let pieceID in this.state.gameState.pieces) {
                        if (enginePieceIDS.indexOf(pieceID) == -1) {
                            toDelete.push(pieceID);
                        }
                    }
                    toDelete.forEach((v: string) => {
                        delete this.state.gameState.pieces[v];
                    })
                }
            }
        }
        if (data['chatMsg']) {
            this.broadcast({sender: client.sessionId, chatMsg: data['chatMsg']})
        }
        if (data['switchLeader']) {
            if (client.sessionId === this.leaderSessionId
                && data['switchLeader'] !== this.leaderSessionId) {
                console.log("Leader wants to switch leaders");
                this.switchLeader(data['switchLeader']);
            }
        }

        if (data['startGame']) {
            if (client.sessionId === this.leaderSessionId) {
                //Initalize game
                //Assign player numbers
                let pNum = 0;
                for (let id in this.state.players) {
                    const player: Player = this.state.players[id];
                    player.playerNum = pNum;
                    pNum++;
                }
                this.broadcast({startGame: true})
            }
        }
        console.log("StateHandlerRoom received message from", client.sessionId, ":", data);
    }

    onDispose() {
        console.log("Dispose StateHandlerRoom");
    }

    private switchLeader(newId: string): void {
        //Make sure this is a valid request
        if (this.state.players[newId]) {
            if (this.leaderSessionId && this.state.players[this.leaderSessionId]) {
                const oldLeader: Player = this.state.players[this.leaderSessionId];
                oldLeader.isLeader = false;

            }
            const newLeader: Player = this.state.players[newId];
            newLeader.isLeader = true;
            this.leaderSessionId = newId;
            this.broadcast({"leaderChange": this.leaderSessionId})
        }
    }

}
