<script>
    import Board from "../chess/Board.svelte";
    import {DisplayMode} from "./DisplayMode";
    //Renders a MovementPattern on a Board
    //Readonly; dispatches events
    export let movementPattern;  //This movementPattern
    export let pieceType; //Which pieceType to display in the center
    export let flipped = false; //View from black?
    export let size = 9; //Size of display in tiles
    export let displayMode;

    let highlighted= {in_check_kings: null, possibleMoves: null, lastTurn: null};
    let center;
    $: center = Math.floor(size/2);
    let gameState;
    $: gameState = {width: size, height:size, tiles: [{x:0,y:0,tile_type:'w'}], pieces:[]};
    //Create tiles based on scaling
    $: gameState.tiles = (() => {
        let tiles = [];
        for (let x = 0; x < gameState.width; x++) {
            for (let y = 0; y < gameState.height; y++) {
                tiles.push({x: x, y: y, tile_type: (x + y) % 2 === 0 ? 'b' : 'w'});
            }
        }
        //Set center tile to the custom piece
        gameState.pieces = [{owner: !flipped ? 0 : 1, x: center, y:center, piece_type:pieceType}]
        let attackTranslateJumps = movementPattern.attackJumps.filter(jump => -1 !== movementPattern.translateJumps.findIndex(value => value[0] === jump[0] && value[1] === jump[1] ));
        if (displayMode === DisplayMode.ALL || displayMode === DisplayMode.ATTACK) {
            for (let jump of movementPattern.attackJumps){
                gameState.pieces = [...gameState.pieces, {x: center + jump[0], y: center + jump[1], piece_type: 'rarrow'}]
            }
        }

        if (displayMode === DisplayMode.ALL || displayMode === DisplayMode.TRANSLATE) {
            for (let jump of movementPattern.translateJumps){
                gameState.pieces = [...gameState.pieces, {x: center + jump[0], y: center + jump[1], piece_type: 'garrow'}]
            }
        }

        if (displayMode === DisplayMode.ALL) {
            for (let jump of attackTranslateJumps){
                gameState.pieces = [...gameState.pieces, {x: center + jump[0], y: center + jump[1], piece_type: 'grarrow'}]
            }
        }

        let attackTranslateSlides = [];
        for (let i=0;i<movementPattern.attackSlideDeltas.length;i++){
            for (let j=0;j<movementPattern.attackSlideDeltas[i].length;j++){
                for (let k=0;k<movementPattern.translateSlideDeltas.length;k++){
                    for (let l=0;l<movementPattern.translateSlideDeltas[k].length;l++){
                        let loc1 = movementPattern.attackSlideDeltas[i][j];
                        let loc2 = movementPattern.translateSlideDeltas[k][l];
                        console.log(loc1);
                        console.log(loc2);
                        if (loc1[0] === loc2[0] && loc1[1] === loc2[1]){
                            attackTranslateSlides.push(loc1);
                        }

                    }
                }
            }
        }

        if (displayMode === DisplayMode.ALL || displayMode === DisplayMode.ATTACK) {
            for (let i=0;i<movementPattern.attackSlideDeltas.length;i++){
                for (let j=0;j<movementPattern.attackSlideDeltas[i].length;j++){
                    let loc = movementPattern.attackSlideDeltas[i][j];
                    gameState.pieces = [...gameState.pieces, {x: center + loc[0], y: center + loc[1], piece_text: i+"-"+j, piece_type: 'srarrow'}]
                }
            }
        }

        if (displayMode === DisplayMode.ALL || displayMode === DisplayMode.TRANSLATE) {
            for (let i=0;i<movementPattern.translateSlideDeltas.length;i++){
                for (let j=0;j<movementPattern.translateSlideDeltas[i].length;j++){
                    let loc = movementPattern.translateSlideDeltas[i][j];
                    gameState.pieces = [...gameState.pieces, {x: center + loc[0], y: center + loc[1], piece_text: i+"-"+j, piece_type: 'sgarrow'}]
                }
            }
        }

        if (displayMode === DisplayMode.ALL){
            for (let loc of attackTranslateSlides){
                gameState.pieces = [...gameState.pieces, {x: center + loc[0], y: center + loc[1], piece_text: "-", piece_type: 'sgrarrow'}]
            }
        }

        return tiles;
    })();

    function generatePossibleTo(pattern){
        let to = [];
        if (pattern.north){
            for (let i=center;i<size;i++){
                to.push([center, i]);
            }
        }
        if (pattern.east){
            for (let i=center;i<size;i++){
                to.push([i, center]);
            }
        }
        if (pattern.south){
            for (let i=center;i >= 0;i--) {
                to.push([center, i]);
            }
        }
        if (pattern.west){
            for (let i=center;i >= 0;i--) {
                to.push([i, center]);
            }
        }
        if (pattern.northeast){
            for (let i=center;i<size;i++){
                to.push([i, i]);
            }
        }
        if (pattern.northwest){
            for (let i=center;i<size;i++){
                to.push([size - i - 1, i]);
            }
        }
        if (pattern.southeast){
            for (let i=center;i >= 0;i--) {
                to.push([size - i - 1, i]);
            }
        }
        if (pattern.southwest){
            for (let i=center;i >= 0;i--) {
                to.push([i, i]);
            }
        }
        return to;
    }

    $: highlighted.etc = (()=>{
        let attacks = []
        if (displayMode === DisplayMode.ALL || displayMode === DisplayMode.ATTACK) {
            attacks = generatePossibleTo(movementPattern.attackSlides).map(function(val){
                return {
                    x: val[0],
                    y: val[1],
                    color: 'rgba(255,27,96,0.4)'
                }
            });
        }

        let translates = [];
        if (displayMode === DisplayMode.ALL || displayMode === DisplayMode.TRANSLATE){
            translates= generatePossibleTo(movementPattern.translateSlides).map(function(val){
                return {
                    x: val[0],
                    y: val[1],
                    color: 'rgba(30,136,255,0.4)'
                }
            });
        }
        return attacks.concat(translates);
    })();

</script>
<Board
        on:tileMouseUp
        on:tileMouseOver
        on:tileMouseDown
        {highlighted}
        {flipped}
        player_num={0}
        gameState={gameState}
/>
