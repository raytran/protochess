//X-Y point
import {Schema, type} from "@colyseus/schema";

export class BoardLocation extends Schema{
    @type("number")
    x:number;
    @type("number")
    y:number;
    constructor(x:number,y:number) {
        super();
        this.x=x;
        this.y=y;
    }
    toString(){
        return this.x + " " + this.y;
    }
}