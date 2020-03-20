import {Schema, type} from "@colyseus/schema";

export class Player extends Schema {
    @type("boolean")
    isLeader= false;

    @type("string")
    id = "";

    @type("string")
    name = "Anon";

    @type("number") //0 for white, 1 for black, 2 for more etc
    playerNum = -1;
}
