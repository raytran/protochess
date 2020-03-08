import * as Colyseus from "colyseus.js";
// @ts-ignore
import {Player} from "protochess-shared";

class ClientHandler {
    client: Colyseus.Client;
    room: Colyseus.Room | null;

    constructor() {
        this.client = new Colyseus.Client('ws://localhost:2567');
        this.room = null;
    }

    joinById(roomId: string) {
        let promise = this.client.joinById(roomId);
        promise.then((newRoom) => {
            this.updateRoom(newRoom)
        }).catch(e => {

        });
        return promise;
    }

    createPrivateRoom() {
        let promise = this.client.create('my_room', {'private': true});
        promise.then(newRoom => {
            this.updateRoom(newRoom);

        }).catch(e => {

        });
        return promise;
    }

    leaveRoom() {
        if (this.room != null) {
            this.room.leave();
            this.room = null;
        }
    }

    private updateRoom(newRoom: Colyseus.Room) {
        this.room = newRoom;
        this.room.state.players.onChange = ()=>{
            console.log(this.room!.state.players);
        };


        this.room.onMessage(message => {
            console.log(message);
        });

        this.room.state.players.onAdd = (player: Player, key: string | number) => {
            console.log("new player");
            console.log(player.name + " has been added at " + key);
            console.log(this.room!.state.players);
        };

        this.room.state.players.onRemove = (player: Player, key: string | number) => {
            console.log(player + "has been removed at" + key);

        }
    }
}

let clientHandler = new ClientHandler();

export default clientHandler;

