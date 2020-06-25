
![Protochess](https://i.imgur.com/5MYfcpe.png)

![Protochess](https://i.imgur.com/6jngcdV.png)

# protochess 
## Online chess variant website written in Svelte/Rust.

The frontend/static website is contained in protochess-front.  

The backend multiplayer websocket server is in protochess-server-rs.

The actual chess logic is in protochess-engine-rs.
All the chess logic, including the chess engine, is written in Rust. The engine is compiled to WebAssembly, with bindings in protochess-engine-wasm, to be served as a static asset. 
