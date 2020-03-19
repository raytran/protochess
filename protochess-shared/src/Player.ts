import {Schema, type} from "@colyseus/schema";

export class Player extends Schema {
    @type("boolean")
    isLeader= false;

    @type("string")
    id = "";

    @type("string")
    name = "Anon";
}
