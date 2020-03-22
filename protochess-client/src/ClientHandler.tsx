import * as Colyseus from "colyseus.js";
// @ts-ignore
import {Player} from "protochess-shared";
import {ChangeEvent} from "react";
import {Simulate} from "react-dom/test-utils";

class ClientHandler {
    private client: Colyseus.Client;
    private room: Colyseus.Room | null;
    private chatListeners: Function[];
    private redirectListeners: Function[];
    private pieceChangeListeners: Function[];
    private playerNumListeners: Function[];
    private pieceDeleteListeners: Function[];
    private gameStateListeners: Function[];
    private playerChangeListeners: Function[];
    private players: any;
    private isLeader:boolean;
    private name:string;
    private playerNum:number | null;

    constructor() {
        this.client = new Colyseus.Client('ws://'+window.location.hostname+':2567');
        this.room = null;
        this.playerNum = null;
        this.playerNumListeners = [];
        this.pieceChangeListeners = [];
        this.pieceDeleteListeners = [];
        this.redirectListeners = [];
        this.gameStateListeners = [];
        this.chatListeners = [];
        this.playerChangeListeners = [];
        this.players = {};
        this.isLeader=false;
        this.name = "";
    }

    getRoom(){
        if (this.room)
            return this.room;
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

    getRoomId() {
        if (this.room != null)
            return this.room.id;
        else return 0;
    }

    getName(){
        return this.name;
    }

    getPlayerNum(){

        return this.playerNum;
    }

    isConnected() {
        return this.room != null;
    }

    addPlayerNumListener(func: Function) {
        this.playerNumListeners.push(func);
    }

    removePlayerNumListener(func: Function) {
        let index = this.playerNumListeners.indexOf(func);
        if (index > -1) {
            this.playerNumListeners.splice(index, 1);
        }
    }

    addGameStateListener(func: Function) {
        this.gameStateListeners.push(func);
    }

    removeGameStateListener(func: Function) {
        let index = this.gameStateListeners.indexOf(func);
        if (index > -1) {
            this.gameStateListeners.splice(index, 1);
        }
    }
    addRedirectListener(func: Function) {
        this.redirectListeners.push(func);
    }

    removeRedirectListener(func: Function) {
        let index = this.redirectListeners.indexOf(func);
        if (index > -1) {
            this.redirectListeners.splice(index, 1);
        }
    }

    addPlayerChangeListener(func: Function) {
        this.playerChangeListeners.push(func);
        if (this.room != null) {
            this.room.state.players.triggerAll();
        }
    }

    addPieceChangeListener(func: Function) {
        this.pieceChangeListeners.push(func);
        if (this.room != null) {
            this.room.state.gameState.pieces.triggerAll();
        }
    }

    removePieceChangeListener(func: Function) {
        let index = this.pieceChangeListeners.indexOf(func);
        if (index > -1) {
            this.pieceChangeListeners.splice(index, 1);
        }
    }


    addPieceDeleteListener(func: Function) {
        this.pieceDeleteListeners.push(func);
        if (this.room != null) {
            this.room.state.gameState.pieces.triggerAll();
        }
    }

    removePieceDeleteListener(func: Function) {
        let index = this.pieceDeleteListeners.indexOf(func);
        if (index > -1) {
            this.pieceDeleteListeners.splice(index, 1);
        }
    }

    removePlayerChangeListener(func: Function) {
        let index = this.playerChangeListeners.indexOf(func);
        if (index > -1) {
            this.playerChangeListeners.splice(index, 1);
        }
    }

    addChatListener(func: Function) {
        this.chatListeners.push(func);
    }

    removeChatListener(func: Function) {
        let index = this.chatListeners.indexOf(func);
        if (index > -1) {
            this.chatListeners.splice(index, 1);
        }
    }
    getPlayers():Player[] {
        return Object.values(this.players);
    }

    sendMessage(data:any){
        if (this.room){
            this.room.send(data);
        }
    }
    getIsLeader(){
        return this.isLeader;
    }

    private updateRoom(newRoom: Colyseus.Room) {
        this.room = newRoom;
        this.players = {};



        this.room.onMessage(message => {
            console.log(message);
            console.log(this.room!.state);
            if (message['leaderChange']){
                this.isLeader = false;
                if (this.room?.sessionId==message['leaderChange']){
                    this.isLeader = true;
                    console.log("I AM THE LEADER");
                    this.updatePlayerChangeListeners();
                }

            }
            if (message['chatMsg']){
                message['senderName'] = this.players[message['sender']].name;
                this.updateChatListeners(message);
            }
            if (message["redirectChallenger"]) {
                for (let i = 0; i < this.redirectListeners.length; i++) {
                    this.redirectListeners[i]("redirectChallenger");
                }
            }

            if (message['startGame']){
                for (let i = 0; i < this.redirectListeners.length; i++) {
                    this.redirectListeners[i]("startGame");
                }


            }
        });

        this.room.state.gameState.onChange = (changes:Colyseus.DataChange[])=>{
            for (let i=0;i<this.gameStateListeners.length;i++){
                this.gameStateListeners[i](changes);
            }
        };

        this.room.state.players.onChange = (changes:Colyseus.DataChange[]) => {
            this.updatePlayerChangeListeners(changes);
        };


        let this_ = this;

        this.room.state.players.onAdd = (player: Player, key: string) => {
            console.log(player.name + " has been added at " + key);
            if (key === this.room?.sessionId){
                this.name = player.name;
            }

            player.onChange = function(changes){
                changes.forEach(change => {
                    if (change.field == 'playerNum'){
                        if (key === this_.room?.sessionId){
                            this_.playerNum = change.value;
                            for (let i=0;i<this_.playerNumListeners.length;i++){
                                this_.playerNumListeners[i]();
                            }
                        }
                    }
                });
            };

            this.players[key] = player;
            this.updatePlayerChangeListeners();
        };

        this.room.state.players.onRemove = (player: Player, key: string | number) => {
            console.log(player + "has been removed at" + key);
            delete this.players[key];
            this.updatePlayerChangeListeners();
        };

        //Need to do this in order to access inner properties
        //i.e. need full sync before attaching listeners
        this.room.onStateChange.once(()=>{
            this.room!.state.gameState.pieces.onChange = (piece:any,key:any) =>{
                console.log(piece + " changed at " +key);
                for (let i=0;i<this.pieceChangeListeners.length;i++){
                    this.pieceChangeListeners[i](piece);
                }
            };

            this.room!.state.gameState.pieces.onRemove = (piece:any,key:any) =>{
                for (let i=0;i<this.pieceDeleteListeners.length;i++) {
                    this.pieceDeleteListeners[i](piece);
                }
            };
        });


    }

    private updatePlayerChangeListeners(changes?:Colyseus.DataChange[]) {
        for (let i = 0; i < this.playerChangeListeners.length; i++) {
            this.playerChangeListeners[i](changes);
        }
    }

    private updateChatListeners(data:any) {
        for (let i = 0; i < this.chatListeners.length; i++) {
            this.chatListeners[i](data);
        }
    }

    switchLeader(newLeaderId: string) {
        this.sendMessage({switchLeader:newLeaderId});
    }

    startGame() {
        this.sendMessage({startGame:true});
    }

    requestMove(move: {id: string; x: number; y: number}) {
        this.sendMessage({'takeTurn':true,move:move});
    }
}

let clientHandler = new ClientHandler();

export default clientHandler;

