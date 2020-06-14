<script>
    import Board from "../chess/Board.svelte";
    export let selectedPiece;

    const ToolType = {
        ATTACK_JUMP : 0,
        TRANSLATE_JUMP : 1,
    }

    let attackPattern = {
        north: false,
        east: false,
        south: false,
        west: false,
        northeast: false,
        northwest: false,
        southeast: false,
        southwest: false,
    }
    let translatePattern = {
        north: false,
        east: false,
        south: false,
        west: false,
        northeast: false,
        northwest: false,
        southeast: false,
        southwest: false,
    }

    let highlighted= {in_check_kings: null, possibleMoves: null, lastTurn: null};



    let center;
    $: center = Math.floor(size/2);
    $: {
        attackPattern;
        gameState;
        let attacks = generatePossibleTo(attackPattern).map(function(val){
            return {
                x: val[0],
                y: val[1],
                color: 'rgba(255,27,96,0.4)'
            }
        });

        let translates = generatePossibleTo(translatePattern).map(function(val){
            return {
                x: val[0],
                y: val[1],
                color: 'rgba(30,136,255,0.4)'
            }
        });
        highlighted.etc = attacks.concat(translates);
    }

    let flipped = false;
    let attackJumps = [];
    let translateJumps = [];
    let size = 9;
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
        gameState.pieces = [{owner: !flipped ? 0 : 1, x: center, y:center, piece_type:selectedPiece}]
        for (let jump of attackJumps){
            gameState.pieces = [...gameState.pieces, {x: center + jump[0], y: center + jump[1], piece_type: 'rarrow'}]
        }

        for (let jump of translateJumps){
            gameState.pieces = [...gameState.pieces, {x: center + jump[0], y: center + jump[1], piece_type: 'garrow'}]
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
    function reset(){
        attackJumps = [];
        translateJumps = [];
        size = 9;
        attackPattern = {
            north: false,
            east: false,
            south: false,
            west: false,
            northeast: false,
            northwest: false,
            southeast: false,
            southwest: false,
        }
        translatePattern = {
            north: false,
            east: false,
            south: false,
            west: false,
            northeast: false,
            northwest: false,
            southeast: false,
            southwest: false,
        }
    }
    let toolSelected = ToolType.ATTACK_JUMP;
    let clicked = false;
    function activateTool(tile){
        console.log(translateJumps);
        let dx = tile.x - center;
        let dy = tile.y - center;
        if (!(tile.x === center && tile.y === center)){
            switch (toolSelected) {
                case ToolType.ATTACK_JUMP:
                    if (attackJumps.findIndex(jmp => jmp[0] === dx && jmp[1] === dy) === -1) {
                        attackJumps = [...attackJumps, [dx, dy]];
                    }else{
                        attackJumps = attackJumps.filter(jmp => !(jmp[0] === dx && jmp[1] === dy));
                    }
                    break;
                case ToolType.TRANSLATE_JUMP:
                    if (translateJumps.findIndex(jmp => jmp[0] === dx && jmp[1] === dy) === -1) {
                        translateJumps = [...translateJumps, [dx, dy]];
                    }else{
                        translateJumps = translateJumps.filter(jmp => !(jmp[0] === dx && jmp[1] === dy));
                    }
                    break;
            }
        }
    }


</script>
<style>
    fieldset{
    }
</style>

<div style="background-color: white">
    <h1> Piece Editor </h1>
    <div style="background-color: white; justify-content:center; font-size: 1em; display: flex; flex-direction: row">
        <fieldset style="margin-right: 2em">
            <button on:click={()=>console.log("bruh")}>Save Changes</button>
            <button on:click={reset}>Reset</button>
            <hr>
            <label>
                <input type=checkbox bind:checked = {flipped}>
                <b>View as black</b>
            </label>
            <hr>
            <b>Viewport Size</b>
            <label>
                <input type=number bind:value={size} min=1 max=32>
                <input step=2 type=range bind:value={size} min=1 max=32>
            </label>



        </fieldset>
        <div on:mouseleave={()=> clicked = false} id="pieceEditor" style="position:relative; width: 30em; height: 30em">
            <Board
                    on:tileMouseUp={()=> clicked = false}
                    on:tileMouseOver={e => (clicked) ?  activateTool(e.detail) : ""}
                            on:tileMouseDown={e => {clicked = true; activateTool(e.detail);}}
                        {highlighted}
                        {flipped}
                            player_num={0}
                            gameState={gameState}
                            />
        </div>


        <div style="display: flex; flex-direction: column;">
            <div style="text-align: center">
                <h1>Moves</h1>
            </div>
            <label>
                <input type=radio value={ToolType.ATTACK_JUMP} bind:group={toolSelected}>
                Attack Jumps
            </label>
            <label>
                <input type=radio value={ToolType.TRANSLATE_JUMP} bind:group={toolSelected}>
                Translate Jumps
            </label>
            <div style="display: flex; flex-direction: row">
                <div>
                    <fieldset>
                        <legend>North</legend>
                        <label>
                            <input type=checkbox bind:checked={attackPattern.north}>
                            Attack
                        </label>
                        <label>
                            <input type=checkbox bind:checked={translatePattern.north}>
                            Translate
                        </label>
                    </fieldset>
                    <fieldset>
                        <legend>East</legend>
                        <label>
                            <input type=checkbox bind:checked={attackPattern.east}>
                            Attack
                        </label>
                        <label>
                            <input type=checkbox bind:checked={translatePattern.east}>
                            Translate
                        </label>
                    </fieldset>

                    <fieldset>
                        <legend>South</legend>
                        <label>
                            <input type=checkbox bind:checked={attackPattern.south}>
                            Attack
                        </label>
                        <label>
                            <input type=checkbox bind:checked={translatePattern.south}>
                            Translate
                        </label>
                    </fieldset>

                    <fieldset>
                        <legend>West</legend>
                        <label>
                            <input type=checkbox bind:checked={attackPattern.west}>
                            Attack
                        </label>
                        <label>
                            <input type=checkbox bind:checked={translatePattern.west}>
                            Translate
                        </label>
                    </fieldset>
                </div>
                <div>
                    <fieldset>
                        <legend>Northeast</legend>
                        <label>
                            <input type=checkbox bind:checked={attackPattern.northeast}>
                            Attack
                        </label>
                        <label>
                            <input type=checkbox bind:checked={translatePattern.northeast}>
                            Translate
                        </label>
                    </fieldset>

                    <fieldset>
                        <legend>Northwest</legend>
                        <label>
                            <input type=checkbox bind:checked={attackPattern.northwest}>
                            Attack
                        </label>
                        <label>
                            <input type=checkbox bind:checked={translatePattern.northwest}>
                            Translate
                        </label>
                    </fieldset>

                    <fieldset>
                        <legend>Southeast</legend>
                        <label>
                            <input type=checkbox bind:checked={attackPattern.southeast}>
                            Attack
                        </label>
                        <label>
                            <input type=checkbox bind:checked={translatePattern.southeast}>
                            Translate
                        </label>
                    </fieldset>

                    <fieldset>
                        <legend>Southwest</legend>
                        <label>
                            <input type=checkbox bind:checked={attackPattern.southwest}>
                            Attack
                        </label>
                        <label>
                            <input type=checkbox bind:checked={translatePattern.southwest}>
                            Translate
                        </label>
                    </fieldset>
                </div>
            </div>
        </div>
    </div>
</div>
