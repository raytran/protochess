<script>
    import Board from "../chess/Board.svelte";

    export let selectedPiece;
    export let movementPattern = {
        attackSlides: {
            north: false,
            east: false,
            south: false,
            west: false,
            northeast: false,
            northwest: false,
            southeast: false,
            southwest: false,
        },
        translateSlides: {
            north: false,
            east: false,
            south: false,
            west: false,
            northeast: false,
            northwest: false,
            southeast: false,
            southwest: false,
        },
        attackJumps: [],
        translateJumps: [],
        attackSlideDeltas: [[]],
        translateSlideDeltas: [[]]
    }



    import { createEventDispatcher } from 'svelte';
    const dispatch = createEventDispatcher();

    const DisplayMode = {
        ALL: 0,
        TRANSLATE: 1,
        ATTACK: 2
    }
    const ToolType = {
        ATTACK_JUMP : 0,
        TRANSLATE_JUMP : 1,
        ATTACK_SLIDE: 2,
        TRANSLATE_SLIDE: 3
    }


    let highlighted= {in_check_kings: null, possibleMoves: null, lastTurn: null};
    let center;
    $: center = Math.floor(size/2);
    $: {
        movementPattern;
        gameState;
        let attacks = generatePossibleTo(movementPattern.attackSlides).map(function(val){
            return {
                x: val[0],
                y: val[1],
                color: 'rgba(255,27,96,0.4)'
            }
        });

        let translates = generatePossibleTo(movementPattern.translateSlides).map(function(val){
            return {
                x: val[0],
                y: val[1],
                color: 'rgba(30,136,255,0.4)'
            }
        });
        highlighted.etc = attacks.concat(translates);
    }

    let flipped = false;


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
        let attackTranslateJumps = movementPattern.attackJumps.filter(jump => -1 !== movementPattern.translateJumps.findIndex(value => value[0] === jump[0] && value[1] === jump[1] ));
        if (displayModeSelected === DisplayMode.ALL || displayModeSelected === DisplayMode.ATTACK) {
            for (let jump of movementPattern.attackJumps){
                gameState.pieces = [...gameState.pieces, {x: center + jump[0], y: center + jump[1], piece_type: 'rarrow'}]
            }
        }

        if (displayModeSelected === DisplayMode.ALL || displayModeSelected === DisplayMode.TRANSLATE) {
            for (let jump of movementPattern.translateJumps){
                gameState.pieces = [...gameState.pieces, {x: center + jump[0], y: center + jump[1], piece_type: 'garrow'}]
            }
        }

        if (displayModeSelected === DisplayMode.ALL) {
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



        if (displayModeSelected === DisplayMode.ALL || displayModeSelected === DisplayMode.ATTACK) {
            for (let i=0;i<movementPattern.attackSlideDeltas.length;i++){
                for (let j=0;j<movementPattern.attackSlideDeltas[i].length;j++){
                    let loc = movementPattern.attackSlideDeltas[i][j];
                    gameState.pieces = [...gameState.pieces, {x: center + loc[0], y: center + loc[1], piece_text: i+"-"+j, piece_type: 'srarrow'}]
                }
            }
        }

        if (displayModeSelected === DisplayMode.ALL || displayModeSelected === DisplayMode.TRANSLATE) {
            for (let i=0;i<movementPattern.translateSlideDeltas.length;i++){
                for (let j=0;j<movementPattern.translateSlideDeltas[i].length;j++){
                    let loc = movementPattern.translateSlideDeltas[i][j];
                    gameState.pieces = [...gameState.pieces, {x: center + loc[0], y: center + loc[1], piece_text: i+"-"+j, piece_type: 'sgarrow'}]
                }
            }
        }

        if (displayModeSelected === DisplayMode.ALL){
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
    function reset(){
        size = 9;
        movementPattern = {
            attackSlides: {
                north: false,
                east: false,
                south: false,
                west: false,
                northeast: false,
                northwest: false,
                southeast: false,
                southwest: false,
            },
            translateSlides: {
                north: false,
                east: false,
                south: false,
                west: false,
                northeast: false,
                northwest: false,
                southeast: false,
                southwest: false,
            },
            attackJumps: [],
            translateJumps: [],
            attackSlideDeltas: [[]],
            translateSlideDeltas: [[]]
        }
    }
    let toolSelected = ToolType.ATTACK_JUMP;
    let displayModeSelected = DisplayMode.ALL;
    let clicked = false;
    function activateTool(tile){
        let dx = tile.x - center;
        let dy = tile.y - center;
        if (!(tile.x === center && tile.y === center)){
            switch (toolSelected) {
                case ToolType.ATTACK_JUMP:
                    if (movementPattern.attackJumps.findIndex(jmp => jmp[0] === dx && jmp[1] === dy) === -1) {
                        movementPattern.attackJumps = [...movementPattern.attackJumps, [dx, dy]];
                    }else{
                        movementPattern.attackJumps = movementPattern.attackJumps.filter(jmp => !(jmp[0] === dx && jmp[1] === dy));
                    }
                    break;
                case ToolType.TRANSLATE_JUMP:
                    if (movementPattern.translateJumps.findIndex(jmp => jmp[0] === dx && jmp[1] === dy) === -1) {
                        movementPattern.translateJumps = [...movementPattern.translateJumps, [dx, dy]];
                    }else{
                        movementPattern.translateJumps = movementPattern.translateJumps.filter(jmp => !(jmp[0] === dx && jmp[1] === dy));
                    }
                    break;

                case ToolType.ATTACK_SLIDE:
                    var last_index = movementPattern.attackSlideDeltas.length - 1;
                    if (movementPattern.attackSlideDeltas[last_index].findIndex(jmp => jmp[0] === dx && jmp[1] === dy) !== -1) {
                        movementPattern.attackSlideDeltas[last_index] = movementPattern.attackSlideDeltas[last_index].filter(jmp => !(jmp[0] === dx && jmp[1] === dy));
                        break;
                    }

                    //Insert if not removed
                    movementPattern.attackSlideDeltas[movementPattern.attackSlideDeltas.length - 1]
                            = [...movementPattern.attackSlideDeltas[movementPattern.attackSlideDeltas.length-1], [dx, dy]];
                    break;
                case ToolType.TRANSLATE_SLIDE:
                    var last_index = movementPattern.translateSlideDeltas.length - 1;
                    if (movementPattern.translateSlideDeltas[last_index].findIndex(jmp => jmp[0] === dx && jmp[1] === dy) !== -1) {
                        movementPattern.translateSlideDeltas[last_index] = movementPattern.translateSlideDeltas[last_index].filter(jmp => !(jmp[0] === dx && jmp[1] === dy));
                        break;
                    }

                    //Insert if not removed
                    movementPattern.translateSlideDeltas[movementPattern.translateSlideDeltas.length - 1]
                            = [...movementPattern.translateSlideDeltas[movementPattern.translateSlideDeltas.length-1], [dx, dy]];
                    break;
            }
        }
    }

    function saveChanges(){
        dispatch('saveChanges', {
            pieceType: selectedPiece,
            movementPattern: movementPattern
        });
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
            <button on:click={saveChanges}>Save Changes</button>
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


            <b>Display Mode</b>
            <label>
                <input type=radio value={DisplayMode.ALL} bind:group={displayModeSelected}>
                All
            </label>

            <label>
                <input type=radio value={DisplayMode.ATTACK} bind:group={displayModeSelected}>
                Attacks
            </label>

            <label>
                <input type=radio value={DisplayMode.TRANSLATE} bind:group={displayModeSelected}>
                Translates
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
                <label>
                    <input type=radio value={ToolType.ATTACK_SLIDE} bind:group={toolSelected}>
                    Attack Slide
                </label>
                {#if toolSelected === ToolType.ATTACK_SLIDE}
                    <button on:click={()=> movementPattern.attackSlideDeltas = [...movementPattern.attackSlideDeltas, []]}>New group</button>
                {/if}
            </div>
            <label>
                <input type=radio value={ToolType.TRANSLATE_SLIDE} bind:group={toolSelected}>
                Translate Slide
                {#if toolSelected === ToolType.TRANSLATE_SLIDE}
                    <button on:click={()=> movementPattern.translateSlideDeltas = [...movementPattern.translateSlideDeltas, []]}>New group</button>
                {/if}
            </label>

            <div style="display: flex; flex-direction: row">
                <div>
                    <fieldset>
                        <legend>Slide North</legend>
                        <label>
                            <input type=checkbox bind:checked={movementPattern.attackSlides.north}>
                            Attack
                        </label>
                        <label>
                            <input type=checkbox bind:checked={movementPattern.translateSlides.north}>
                            Translate
                        </label>
                    </fieldset>
                    <fieldset>
                        <legend>Slide East</legend>
                        <label>
                            <input type=checkbox bind:checked={movementPattern.attackSlides.east}>
                            Attack
                        </label>
                        <label>
                            <input type=checkbox bind:checked={movementPattern.translateSlides.east}>
                            Translate
                        </label>
                    </fieldset>

                    <fieldset>
                        <legend>Slide South</legend>
                        <label>
                            <input type=checkbox bind:checked={movementPattern.attackSlides.south}>
                            Attack
                        </label>
                        <label>
                            <input type=checkbox bind:checked={movementPattern.translateSlides.south}>
                            Translate
                        </label>
                    </fieldset>

                    <fieldset>
                        <legend>Slide West</legend>
                        <label>
                            <input type=checkbox bind:checked={movementPattern.attackSlides.west}>
                            Attack
                        </label>
                        <label>
                            <input type=checkbox bind:checked={movementPattern.translateSlides.west}>
                            Translate
                        </label>
                    </fieldset>
                </div>
                <div>
                    <fieldset>
                        <legend>Slide Northeast</legend>
                        <label>
                            <input type=checkbox bind:checked={movementPattern.attackSlides.northeast}>
                            Attack
                        </label>
                        <label>
                            <input type=checkbox bind:checked={movementPattern.translateSlides.northeast}>
                            Translate
                        </label>
                    </fieldset>

                    <fieldset>
                        <legend>Slide Northwest</legend>
                        <label>
                            <input type=checkbox bind:checked={movementPattern.attackSlides.northwest}>
                            Attack
                        </label>
                        <label>
                            <input type=checkbox bind:checked={movementPattern.translateSlides.northwest}>
                            Translate
                        </label>
                    </fieldset>

                    <fieldset>
                        <legend>Slide Southeast</legend>
                        <label>
                            <input type=checkbox bind:checked={movementPattern.attackSlides.southeast}>
                            Attack
                        </label>
                        <label>
                            <input type=checkbox bind:checked={movementPattern.translateSlides.southeast}>
                            Translate
                        </label>
                    </fieldset>

                    <fieldset>
                        <legend>Slide Southwest</legend>
                        <label>
                            <input type=checkbox bind:checked={movementPattern.attackSlides.southwest}>
                            Attack
                        </label>
                        <label>
                            <input type=checkbox bind:checked={movementPattern.translateSlides.southwest}>
                            Translate
                        </label>
                    </fieldset>
                </div>
            </div>
        </div>
    </div>
</div>
