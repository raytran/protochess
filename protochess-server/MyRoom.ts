import { Room, Client } from "colyseus";

export class MyRoom extends Room {

  onCreate (options: any) {
    console.log(options);
    if (options['private']){
        this.setPrivate(true);
        console.log('private room');
    }
    console.log("Room created");
    console.log(this.roomId);
  }

  onJoin (client: Client, options: any) {
    console.log("Client joined");
  }

  onMessage (client: Client, message: any) {
  }

  onLeave (client: Client, consented: boolean) {
  }

  onDispose() {
      console.log("Room deleted " +this.roomId);
  }

}
