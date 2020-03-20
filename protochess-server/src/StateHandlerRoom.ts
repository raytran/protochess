import {Client, Room} from "colyseus";
import {Movement,BoardLocation,State,Player} from "protochess-shared"

export class StateHandlerRoom extends Room<State> {
    maxClients = 4;
    leaderSessionId: string | null = null;
    didRedirect = false;

    private switchLeader(newId:string):void{
        //Make sure this is a valid request
        if (this.state.players[newId]){
            if (this.leaderSessionId && this.state.players[this.leaderSessionId]){
                const oldLeader: Player = this.state.players[this.leaderSessionId];
                oldLeader.isLeader = false;

            }
            const newLeader: Player = this.state.players[newId];
            newLeader.isLeader = true;
            this.leaderSessionId = newId;
            this.broadcast({"leaderChange":this.leaderSessionId})
        }
    }
    onCreate(options: any) {
        console.log("StateHandlerRoom created!", options);

        this.setState(new State());
    }

    onJoin(client: Client) {
        console.log(client.sessionId + "join");
        if (this.clients.length == 1){
            this.state.createPlayer(client.sessionId,true,"");
            this.switchLeader(client.sessionId);
        }else{
            this.state.createPlayer(client.sessionId,false,"");
        }
        if (this.clients.length == 2 && !this.didRedirect) {
            this.broadcast({redirectChallenger:true});
            this.didRedirect = true;
        }
    }

    onLeave(client: Client) {
        console.log(client.sessionId + "leave");
        this.state.removePlayer(client.sessionId);
        if (client.sessionId == this.leaderSessionId){
            for (let id in this.state.players){
                this.switchLeader(id);
                console.log("Leader updated");
                break;
            }
        }
    }

    onMessage(client: Client, data: any) {
        if (data['takeTurn']){
            console.log(data['move'])
            if (this.state.gameState.whosTurn == this.state.players[client.sessionId].playerNum
                && this.state.gameState.pieces[data['move']['id']]){
                    let piece = this.state.gameState.pieces[data['move']['id']];
                    let startX = piece.location.x;
                    let startY = piece.location.y;
                    let endX = data['move']['x'];
                    let endY = data['move']['y'];
                    try {
                        let startLoc = new BoardLocation(startX, startY);
                        let endLoc = new BoardLocation(endX, endY);
                        let movement = new Movement(startLoc, endLoc);
                        this.state.gameState.takeTurn(movement);
                    }catch (e) {
                    }
            }
        }
        if (data['chatMsg']){
            this.broadcast({sender:client.sessionId, chatMsg:data['chatMsg']})
        }
        if (data['switchLeader']){
            if (client.sessionId === this.leaderSessionId
                && data['switchLeader'] !== this.leaderSessionId){
                console.log("Leader wants to switch leaders");
                this.switchLeader(data['switchLeader']);
            }
        }

        if (data['startGame']){
            if (client.sessionId === this.leaderSessionId){
                //Initalize game
                //Assign player numbers
                this.state.gameState.assignPlayerNumbers(this.state.players);
                this.broadcast({startGame:true})
            }
        }
        console.log("StateHandlerRoom received message from", client.sessionId, ":", data);
    }

    onDispose() {
        console.log("Dispose StateHandlerRoom");
    }

}
