
![Protochess](https://i.imgur.com/5MYfcpe.png)

![Protochess](https://i.imgur.com/6jngcdV.png) 

![Protochess](https://i.imgur.com/Drv9MgG.png)

# Protochess 
Protochess is an online chess website designed to let you customize your own chess boards/pieces. Want a piece that can move like a knight + queen? Sure. Want to play on a 16x16 sized board? Impractical but you can do it!

## Built with
Rust for the backend/chess logic/multiplayer server

Svelte for the single page app frontend

## Project structure

The frontend/static website is contained in protochess-front.  

The backend multiplayer websocket server is in protochess-server-rs.

Common shared data structures between the front/backend are in protochess-common

The actual chess logic/chess engine is in protochess-engine-rs.

## Running locally with docker

Included in the base directory is a Dockerfile that compiles the frontend and launches the webserver.
 
```
git clone https://github.com/raytran/protochess
cd protochess
docker build -t protochess .
docker run -p 3030:3030 protochess
```
This will launch the chess page at localhost:3030

## Contact
You can email me (Raymond Tran) here: raytran@mit.edu
