<script>
    import Tile from '../Chess/Tile.svelte'
    import {createEventDispatcher, onMount} from 'svelte';
    import Board from "../Chess/Board.svelte";
    import PieceEditor from "./PieceEditor.svelte";
    import MovementPatternDisplayBar from "../MovementPatternDisplayBar/MovementPatternDisplayBar.svelte";

    let rawData = JSON.parse("{\"type\":\"GameState\",\"content\":{\"width\":8,\"height\":8,\"winner\":null,\"to_move\":0,\"to_move_in_check\":false,\"in_check_kings\":null,\"last_turn\":null,\"tiles\":[{\"x\":0,\"y\":0,\"tile_type\":\"b\"},{\"x\":0,\"y\":1,\"tile_type\":\"w\"},{\"x\":0,\"y\":2,\"tile_type\":\"b\"},{\"x\":0,\"y\":3,\"tile_type\":\"w\"},{\"x\":0,\"y\":4,\"tile_type\":\"b\"},{\"x\":0,\"y\":5,\"tile_type\":\"w\"},{\"x\":0,\"y\":6,\"tile_type\":\"b\"},{\"x\":0,\"y\":7,\"tile_type\":\"w\"},{\"x\":1,\"y\":0,\"tile_type\":\"w\"},{\"x\":1,\"y\":1,\"tile_type\":\"b\"},{\"x\":1,\"y\":2,\"tile_type\":\"w\"},{\"x\":1,\"y\":3,\"tile_type\":\"b\"},{\"x\":1,\"y\":4,\"tile_type\":\"w\"},{\"x\":1,\"y\":5,\"tile_type\":\"b\"},{\"x\":1,\"y\":6,\"tile_type\":\"w\"},{\"x\":1,\"y\":7,\"tile_type\":\"b\"},{\"x\":2,\"y\":0,\"tile_type\":\"b\"},{\"x\":2,\"y\":1,\"tile_type\":\"w\"},{\"x\":2,\"y\":2,\"tile_type\":\"b\"},{\"x\":2,\"y\":3,\"tile_type\":\"w\"},{\"x\":2,\"y\":4,\"tile_type\":\"b\"},{\"x\":2,\"y\":5,\"tile_type\":\"w\"},{\"x\":2,\"y\":6,\"tile_type\":\"b\"},{\"x\":2,\"y\":7,\"tile_type\":\"w\"},{\"x\":3,\"y\":0,\"tile_type\":\"w\"},{\"x\":3,\"y\":1,\"tile_type\":\"b\"},{\"x\":3,\"y\":2,\"tile_type\":\"w\"},{\"x\":3,\"y\":3,\"tile_type\":\"b\"},{\"x\":3,\"y\":4,\"tile_type\":\"w\"},{\"x\":3,\"y\":5,\"tile_type\":\"b\"},{\"x\":3,\"y\":6,\"tile_type\":\"w\"},{\"x\":3,\"y\":7,\"tile_type\":\"b\"},{\"x\":4,\"y\":0,\"tile_type\":\"b\"},{\"x\":4,\"y\":1,\"tile_type\":\"w\"},{\"x\":4,\"y\":2,\"tile_type\":\"b\"},{\"x\":4,\"y\":3,\"tile_type\":\"w\"},{\"x\":4,\"y\":4,\"tile_type\":\"b\"},{\"x\":4,\"y\":5,\"tile_type\":\"w\"},{\"x\":4,\"y\":6,\"tile_type\":\"b\"},{\"x\":4,\"y\":7,\"tile_type\":\"w\"},{\"x\":5,\"y\":0,\"tile_type\":\"w\"},{\"x\":5,\"y\":1,\"tile_type\":\"b\"},{\"x\":5,\"y\":2,\"tile_type\":\"w\"},{\"x\":5,\"y\":3,\"tile_type\":\"b\"},{\"x\":5,\"y\":4,\"tile_type\":\"w\"},{\"x\":5,\"y\":5,\"tile_type\":\"b\"},{\"x\":5,\"y\":6,\"tile_type\":\"w\"},{\"x\":5,\"y\":7,\"tile_type\":\"b\"},{\"x\":6,\"y\":0,\"tile_type\":\"b\"},{\"x\":6,\"y\":1,\"tile_type\":\"w\"},{\"x\":6,\"y\":2,\"tile_type\":\"b\"},{\"x\":6,\"y\":3,\"tile_type\":\"w\"},{\"x\":6,\"y\":4,\"tile_type\":\"b\"},{\"x\":6,\"y\":5,\"tile_type\":\"w\"},{\"x\":6,\"y\":6,\"tile_type\":\"b\"},{\"x\":6,\"y\":7,\"tile_type\":\"w\"},{\"x\":7,\"y\":0,\"tile_type\":\"w\"},{\"x\":7,\"y\":1,\"tile_type\":\"b\"},{\"x\":7,\"y\":2,\"tile_type\":\"w\"},{\"x\":7,\"y\":3,\"tile_type\":\"b\"},{\"x\":7,\"y\":4,\"tile_type\":\"w\"},{\"x\":7,\"y\":5,\"tile_type\":\"b\"},{\"x\":7,\"y\":6,\"tile_type\":\"w\"},{\"x\":7,\"y\":7,\"tile_type\":\"b\"}],\"pieces\":[{\"owner\":0,\"x\":4,\"y\":0,\"piece_type\":\"k\"},{\"owner\":0,\"x\":3,\"y\":0,\"piece_type\":\"q\"},{\"owner\":0,\"x\":2,\"y\":0,\"piece_type\":\"b\"},{\"owner\":0,\"x\":5,\"y\":0,\"piece_type\":\"b\"},{\"owner\":0,\"x\":1,\"y\":0,\"piece_type\":\"n\"},{\"owner\":0,\"x\":6,\"y\":0,\"piece_type\":\"n\"},{\"owner\":0,\"x\":0,\"y\":0,\"piece_type\":\"r\"},{\"owner\":0,\"x\":7,\"y\":0,\"piece_type\":\"r\"},{\"owner\":0,\"x\":0,\"y\":1,\"piece_type\":\"p\"},{\"owner\":0,\"x\":1,\"y\":1,\"piece_type\":\"p\"},{\"owner\":0,\"x\":2,\"y\":1,\"piece_type\":\"p\"},{\"owner\":0,\"x\":3,\"y\":1,\"piece_type\":\"p\"},{\"owner\":0,\"x\":4,\"y\":1,\"piece_type\":\"p\"},{\"owner\":0,\"x\":5,\"y\":1,\"piece_type\":\"p\"},{\"owner\":0,\"x\":6,\"y\":1,\"piece_type\":\"p\"},{\"owner\":0,\"x\":7,\"y\":1,\"piece_type\":\"p\"},{\"owner\":1,\"x\":4,\"y\":7,\"piece_type\":\"k\"},{\"owner\":1,\"x\":3,\"y\":7,\"piece_type\":\"q\"},{\"owner\":1,\"x\":2,\"y\":7,\"piece_type\":\"b\"},{\"owner\":1,\"x\":5,\"y\":7,\"piece_type\":\"b\"},{\"owner\":1,\"x\":1,\"y\":7,\"piece_type\":\"n\"},{\"owner\":1,\"x\":6,\"y\":7,\"piece_type\":\"n\"},{\"owner\":1,\"x\":0,\"y\":7,\"piece_type\":\"r\"},{\"owner\":1,\"x\":7,\"y\":7,\"piece_type\":\"r\"},{\"owner\":1,\"x\":0,\"y\":6,\"piece_type\":\"p\"},{\"owner\":1,\"x\":1,\"y\":6,\"piece_type\":\"p\"},{\"owner\":1,\"x\":2,\"y\":6,\"piece_type\":\"p\"},{\"owner\":1,\"x\":3,\"y\":6,\"piece_type\":\"p\"},{\"owner\":1,\"x\":4,\"y\":6,\"piece_type\":\"p\"},{\"owner\":1,\"x\":5,\"y\":6,\"piece_type\":\"p\"},{\"owner\":1,\"x\":6,\"y\":6,\"piece_type\":\"p\"},{\"owner\":1,\"x\":7,\"y\":6,\"piece_type\":\"p\"}],\"movement_patterns\":{}}}");
    export let gameState = rawData['content'];

    let ToolType = {
        TILE: 1,
        PIECE: 2,
    };

    let PieceType = {
        PAWN: 'p',
        BISHOP: 'b',
        KNIGHT: 'n',
        QUEEN: 'q',
        KING: 'k',
        ROOK: 'r',
        CUSTOM: '#',
    }

    let showPieceEditor = false;
    let customPieceCharSelected = 'a';
    let unregisteredChars = 'acdefghijlmostuvwxyz'.split('');
    let registeredChars = [];

    let pieceSelected = PieceType.PAWN;
    let pieceOwnerSelected = 0;
    let toolSelected = ToolType.TILE;
    let dimensions = {width: gameState.width, height: gameState.height};

    $: gameStateJSON = (()=>{
        return JSON.stringify(gameState);
    })();
    let restricted = [];
    let flipped = false;
    $: gameState.width = (() => {
        gameState.pieces = gameState.pieces.filter(pce => pce.x < gameState.width);
        return dimensions.width
    })();
    $: gameState.height = (() => {
        gameState.pieces = gameState.pieces.filter(pce => pce.y < gameState.height);
        return dimensions.height
    })();
    $: gameState.tiles = (() => {
        let tiles = [];
        for (let x = 0; x < dimensions.width; x++) {
            for (let y = 0; y < dimensions.height; y++) {
                let allowed = true;
                for (let r of restricted) {
                    if (r.x === x && r.y === y) {
                        allowed = false;
                        break;
                    }
                }
                if (allowed) {
                    tiles.push({x: x, y: y, tile_type: (x + y) % 2 === 0 ? 'b' : 'w'});
                } else {
                    tiles.push({x: x, y: y, tile_type: 'x'});
                }
            }
        }
        return tiles;
    })();

    function reset() {
        let rawData = JSON.parse("{\"type\":\"GameState\",\"content\":{\"width\":8,\"height\":8,\"winner\":null,\"to_move\":0,\"to_move_in_check\":false,\"in_check_kings\":null,\"last_turn\":null,\"tiles\":[{\"x\":0,\"y\":0,\"tile_type\":\"b\"},{\"x\":0,\"y\":1,\"tile_type\":\"w\"},{\"x\":0,\"y\":2,\"tile_type\":\"b\"},{\"x\":0,\"y\":3,\"tile_type\":\"w\"},{\"x\":0,\"y\":4,\"tile_type\":\"b\"},{\"x\":0,\"y\":5,\"tile_type\":\"w\"},{\"x\":0,\"y\":6,\"tile_type\":\"b\"},{\"x\":0,\"y\":7,\"tile_type\":\"w\"},{\"x\":1,\"y\":0,\"tile_type\":\"w\"},{\"x\":1,\"y\":1,\"tile_type\":\"b\"},{\"x\":1,\"y\":2,\"tile_type\":\"w\"},{\"x\":1,\"y\":3,\"tile_type\":\"b\"},{\"x\":1,\"y\":4,\"tile_type\":\"w\"},{\"x\":1,\"y\":5,\"tile_type\":\"b\"},{\"x\":1,\"y\":6,\"tile_type\":\"w\"},{\"x\":1,\"y\":7,\"tile_type\":\"b\"},{\"x\":2,\"y\":0,\"tile_type\":\"b\"},{\"x\":2,\"y\":1,\"tile_type\":\"w\"},{\"x\":2,\"y\":2,\"tile_type\":\"b\"},{\"x\":2,\"y\":3,\"tile_type\":\"w\"},{\"x\":2,\"y\":4,\"tile_type\":\"b\"},{\"x\":2,\"y\":5,\"tile_type\":\"w\"},{\"x\":2,\"y\":6,\"tile_type\":\"b\"},{\"x\":2,\"y\":7,\"tile_type\":\"w\"},{\"x\":3,\"y\":0,\"tile_type\":\"w\"},{\"x\":3,\"y\":1,\"tile_type\":\"b\"},{\"x\":3,\"y\":2,\"tile_type\":\"w\"},{\"x\":3,\"y\":3,\"tile_type\":\"b\"},{\"x\":3,\"y\":4,\"tile_type\":\"w\"},{\"x\":3,\"y\":5,\"tile_type\":\"b\"},{\"x\":3,\"y\":6,\"tile_type\":\"w\"},{\"x\":3,\"y\":7,\"tile_type\":\"b\"},{\"x\":4,\"y\":0,\"tile_type\":\"b\"},{\"x\":4,\"y\":1,\"tile_type\":\"w\"},{\"x\":4,\"y\":2,\"tile_type\":\"b\"},{\"x\":4,\"y\":3,\"tile_type\":\"w\"},{\"x\":4,\"y\":4,\"tile_type\":\"b\"},{\"x\":4,\"y\":5,\"tile_type\":\"w\"},{\"x\":4,\"y\":6,\"tile_type\":\"b\"},{\"x\":4,\"y\":7,\"tile_type\":\"w\"},{\"x\":5,\"y\":0,\"tile_type\":\"w\"},{\"x\":5,\"y\":1,\"tile_type\":\"b\"},{\"x\":5,\"y\":2,\"tile_type\":\"w\"},{\"x\":5,\"y\":3,\"tile_type\":\"b\"},{\"x\":5,\"y\":4,\"tile_type\":\"w\"},{\"x\":5,\"y\":5,\"tile_type\":\"b\"},{\"x\":5,\"y\":6,\"tile_type\":\"w\"},{\"x\":5,\"y\":7,\"tile_type\":\"b\"},{\"x\":6,\"y\":0,\"tile_type\":\"b\"},{\"x\":6,\"y\":1,\"tile_type\":\"w\"},{\"x\":6,\"y\":2,\"tile_type\":\"b\"},{\"x\":6,\"y\":3,\"tile_type\":\"w\"},{\"x\":6,\"y\":4,\"tile_type\":\"b\"},{\"x\":6,\"y\":5,\"tile_type\":\"w\"},{\"x\":6,\"y\":6,\"tile_type\":\"b\"},{\"x\":6,\"y\":7,\"tile_type\":\"w\"},{\"x\":7,\"y\":0,\"tile_type\":\"w\"},{\"x\":7,\"y\":1,\"tile_type\":\"b\"},{\"x\":7,\"y\":2,\"tile_type\":\"w\"},{\"x\":7,\"y\":3,\"tile_type\":\"b\"},{\"x\":7,\"y\":4,\"tile_type\":\"w\"},{\"x\":7,\"y\":5,\"tile_type\":\"b\"},{\"x\":7,\"y\":6,\"tile_type\":\"w\"},{\"x\":7,\"y\":7,\"tile_type\":\"b\"}],\"pieces\":[{\"owner\":0,\"x\":4,\"y\":0,\"piece_type\":\"k\"},{\"owner\":0,\"x\":3,\"y\":0,\"piece_type\":\"q\"},{\"owner\":0,\"x\":2,\"y\":0,\"piece_type\":\"b\"},{\"owner\":0,\"x\":5,\"y\":0,\"piece_type\":\"b\"},{\"owner\":0,\"x\":1,\"y\":0,\"piece_type\":\"n\"},{\"owner\":0,\"x\":6,\"y\":0,\"piece_type\":\"n\"},{\"owner\":0,\"x\":0,\"y\":0,\"piece_type\":\"r\"},{\"owner\":0,\"x\":7,\"y\":0,\"piece_type\":\"r\"},{\"owner\":0,\"x\":0,\"y\":1,\"piece_type\":\"p\"},{\"owner\":0,\"x\":1,\"y\":1,\"piece_type\":\"p\"},{\"owner\":0,\"x\":2,\"y\":1,\"piece_type\":\"p\"},{\"owner\":0,\"x\":3,\"y\":1,\"piece_type\":\"p\"},{\"owner\":0,\"x\":4,\"y\":1,\"piece_type\":\"p\"},{\"owner\":0,\"x\":5,\"y\":1,\"piece_type\":\"p\"},{\"owner\":0,\"x\":6,\"y\":1,\"piece_type\":\"p\"},{\"owner\":0,\"x\":7,\"y\":1,\"piece_type\":\"p\"},{\"owner\":1,\"x\":4,\"y\":7,\"piece_type\":\"k\"},{\"owner\":1,\"x\":3,\"y\":7,\"piece_type\":\"q\"},{\"owner\":1,\"x\":2,\"y\":7,\"piece_type\":\"b\"},{\"owner\":1,\"x\":5,\"y\":7,\"piece_type\":\"b\"},{\"owner\":1,\"x\":1,\"y\":7,\"piece_type\":\"n\"},{\"owner\":1,\"x\":6,\"y\":7,\"piece_type\":\"n\"},{\"owner\":1,\"x\":0,\"y\":7,\"piece_type\":\"r\"},{\"owner\":1,\"x\":7,\"y\":7,\"piece_type\":\"r\"},{\"owner\":1,\"x\":0,\"y\":6,\"piece_type\":\"p\"},{\"owner\":1,\"x\":1,\"y\":6,\"piece_type\":\"p\"},{\"owner\":1,\"x\":2,\"y\":6,\"piece_type\":\"p\"},{\"owner\":1,\"x\":3,\"y\":6,\"piece_type\":\"p\"},{\"owner\":1,\"x\":4,\"y\":6,\"piece_type\":\"p\"},{\"owner\":1,\"x\":5,\"y\":6,\"piece_type\":\"p\"},{\"owner\":1,\"x\":6,\"y\":6,\"piece_type\":\"p\"},{\"owner\":1,\"x\":7,\"y\":6,\"piece_type\":\"p\"}],\"movement_patterns\":{}}}");
        gameState = rawData['content'];
        dimensions = {width: 8, height: 8};
        restricted = [];
    }

    let clicked = false;

    function toggleRestricted(tile) {
        let i = restricted.findIndex(obj => obj.x === tile.x && obj.y === tile.y);
        if (i !== -1) {
            //Not here, insert
            restricted = restricted.filter(item => !(item.x === tile.x && item.y === tile.y));
        } else {
            restricted = [...restricted, {x: tile.x, y: tile.y}];
        }
    }

    function togglePiece(tile) {
        if (gameState.pieces.findIndex(pce => pce.x === tile.x && pce.y === tile.y) === -1) {
            if (pieceSelected !== PieceType.CUSTOM) {
                gameState.pieces = [...gameState.pieces, {
                    owner: pieceOwnerSelected,
                    x: tile.x,
                    y: tile.y,
                    piece_type: pieceSelected
                }];
            } else {
                gameState.pieces = [...gameState.pieces, {
                    owner: pieceOwnerSelected,
                    x: tile.x,
                    y: tile.y,
                    piece_type: customPieceCharSelected
                }];
            }
        } else {
            gameState.pieces = gameState.pieces.filter(pce => !(pce.x === tile.x && pce.y === tile.y));
        }
    }

    function activateTool(tile) {
        switch (toolSelected) {
            case ToolType.TILE:
                toggleRestricted(tile);
                break;
            case ToolType.PIECE:
                togglePiece(tile);
                break;
        }
    }

    function isMovementPatternDefault(mp) {
        for (let dir in mp.attackSlides) {
            if (mp.attackSlides[dir]) return false;
        }
        for (let dir in mp.translateSlides) {
            if (mp.translateSlides[dir]) return false;
        }
        if (mp.attackJumps.length !== 0) return false;
        if (mp.translateJumps.length !== 0) return false;

        //Check if attackSlideDeltas is empty
        if (!mp.attackSlideDeltas.every(function (a) {
            return !a.length
        })) return false;
        if (!mp.translateSlideDeltas.every(function (a) {
            return !a.length
        })) return false;
        return true;
    }

    function handlePieceEditor(e) {
        //Check if !default
        let mp = e.detail.movementPattern;
        gameState.movement_patterns = gameState.movement_patterns;
        if (!isMovementPatternDefault(mp)) {
            let index = unregisteredChars.findIndex(elem => elem === e.detail.pieceType);
            if (-1 !== index) {
                unregisteredChars.splice(index, 1);
                unregisteredChars = unregisteredChars;
                registeredChars = [...registeredChars, e.detail.pieceType];
            }
            gameState.movement_patterns[e.detail.pieceType] = e.detail.movementPattern;
        } else {
            //If a piece has been reset
            let index = registeredChars.findIndex(elem => elem === e.detail.pieceType);
            if (-1 !== index) {
                //Then move back to unregistered
                registeredChars.splice(index, 1)
                registeredChars = registeredChars;
                unregisteredChars = [...unregisteredChars, e.detail.pieceType];
                delete gameState.movement_patterns[e.detail.pieceType]
            }
        }
        showPieceEditor = false;

    }
    const dispatch = createEventDispatcher();
    function onSaveChanges(){
        dispatch("saveChanges", gameState);
    }
</script>

<style>
    #pieceEditorWrapper{
        position: absolute;
        top:0;
        left:0;
        right:0;
        margin: 0 auto;
        width: 100%;
        background-color: rgba(0,0,0,0.3);
        height: 100%;
    }

    fieldset{
        border: 0;
        text-align: left;
    }

    #container{
        display: grid;
        justify-items: center;
        column-gap: 1em;
        row-gap: 1em;
        grid-template-areas: 'left-control board board right-control';
        grid-template-columns: repeat(4,  1fr);
        font-size: 1em;
    }
    @media (max-width: 1200px) {
        #container {
            grid-template-columns: repeat(2,  1fr);
            grid-template-areas:
                    'board        board'
                    'left-control right-control'
        }
    }

    @media (max-width: 650px) {
        #container {
            grid-template-columns: 1fr;
            grid-template-areas:
                    'board'
                    'left-control'
                    'right-control'
        }
    }

    #leftControl{
        grid-area: left-control;
        width: 100%;
        max-width: 350px;
        padding: 1em;
        box-shadow: 0 10px 20px rgba(0,0,0,0.19), 0 6px 6px rgba(0,0,0,0.23);
        text-align: center;
    }
    #boardWrapper {
        max-width: 700px;
        grid-area: board;
        width: 100%;
    }

    #rightControl {
        grid-area: right-control;
        width: 100%;
        max-width: 400px;
        padding: 1em;
        box-shadow: 0 10px 20px rgba(0,0,0,0.19), 0 6px 6px rgba(0,0,0,0.23);
    }
    .customCharEntry {
        display: flex;
    }
    .customCharPic{
        width: 3em;
        height: 3em;
    }
    #customCharList {
        width: 10em;
        overflow: auto;
        height: 10vh;
    }

    #movementPatternDisplayBarWrapper {
        height: 20em;
        overflow: scroll;
    }
</style>

<div id="container">
    <div id="leftControl">
        <fieldset>
            <button on:click={reset}>Reset</button>
            <button on:click={onSaveChanges}>Save Changes</button>
            <hr>
            <label>
                <input type=checkbox bind:checked = {flipped}>
                <b>View as black</b>
            </label>
            <hr>
            <b>Board Width</b>
            <label>
                <input type=number bind:value={dimensions.width} min=1 max=16>
                <input type=range bind:value={dimensions.width} min=1 max=16>
            </label>
            <b>Board Height</b>
            <label>
                <input type=number bind:value={dimensions.height} min=1 max=16>
                <input type=range bind:value={dimensions.height} min=1 max=16>
            </label>

            <b>JSON</b>
            <label>
                <textarea id="gameStateJSONDisplay" readonly value={gameStateJSON}></textarea>
            </label>

        </fieldset>
        <b>Registered Movement Patterns:</b>
        <div id="movementPatternDisplayBarWrapper">
            <MovementPatternDisplayBar movementPatterns={gameState.movement_patterns} />
        </div>
    </div>
    <div id="boardWrapper">
        <Board
                {flipped}
                player_num={0}
                gameState={gameState}
                on:tileMouseUp={()=> clicked = false}
                on:tileMouseOver={e => (clicked) ?  activateTool(e.detail) : ""}
                        on:tileMouseDown={e => {clicked = true; activateTool(e.detail);}} />
    </div>
        <div id="rightControl">
            <fieldset >
                <legend><b>Select Tool</b></legend>
                <br>
                <label>
                    <input type=radio bind:group={toolSelected} value={ToolType.TILE}/>
                    Toggle tiles
                </label>
                <label>
                    <input type=radio bind:group={toolSelected} value={ToolType.PIECE}/>
                    Place piece
                </label>
                {#if toolSelected === ToolType.PIECE}
                    <hr>
                    <b>Tool Options</b>
                    <label>
                        <input type=radio bind:group={pieceOwnerSelected} value={0}/>
                        White
                    </label>
                    <label>
                        <input type=radio bind:group={pieceOwnerSelected} value={1}/>
                        Black
                    </label>
                    <hr>

                    <label>
                        <input type=radio bind:group={pieceSelected} value={PieceType.PAWN}/>
                        Pawn
                    </label>

                    <label>
                        <input type=radio bind:group={pieceSelected} value={PieceType.KNIGHT}/>
                        Knight
                    </label>

                    <label>
                        <input type=radio bind:group={pieceSelected} value={PieceType.BISHOP}/>
                        Bishop
                    </label>

                    <label>
                        <input type=radio bind:group={pieceSelected} value={PieceType.QUEEN}/>
                        Queen
                    </label>

                    <label>
                        <input type=radio bind:group={pieceSelected} value={PieceType.KING}/>
                        King
                    </label>

                    <label>
                        <input type=radio bind:group={pieceSelected} value={PieceType.CUSTOM}/>
                        Custom
                    </label>

                    {#if pieceSelected === PieceType.CUSTOM}
                        <div>
                            Custom Piece:
                            <div id="customCharList">
                                {#each registeredChars as char}
                                    <div class=customCharEntry style="background-color:
                            {customPieceCharSelected === char ? 'rgba(0,0,255,0.6)' : 'rgba(0,0,255,0.3)'};"
                                         on:click={()=> customPieceCharSelected = char}>
                                        <div> Ready </div>
                                        <div class="customCharPic">
                                            <img src={'/images/chess_pieces/' + (pieceOwnerSelected === 0 ? char.toUpperCase() : char) + '.svg'}/>
                                        </div>
                                    </div>
                                {/each}

                                {#each unregisteredChars as char}
                                    <div class=customCharEntry style="background-color:
                                {customPieceCharSelected === char ? 'rgba(255,00,0,0.6)' :  'rgba(255,0,0,0.3)'};"
                                         on:click={()=> customPieceCharSelected = char}>
                                        <div> ??? </div>
                                        <div class="customCharPic">
                                            <img src={'/images/chess_pieces/' + (pieceOwnerSelected === 0 ? char.toUpperCase() : char) + '.svg'}/>
                                        </div>
                                    </div>
                                {/each}
                            </div>
                        </div>
                        <button on:click={()=> showPieceEditor = true}>
                            {registeredChars.includes(customPieceCharSelected) ? 'Edit movement' : 'Set movement'}
                        </button>
                    {/if}
                {/if}
            </fieldset>
        </div>

</div>
{#if showPieceEditor}
    <div id=pieceEditorWrapper>
        <PieceEditor on:saveChanges={handlePieceEditor}
                     pieceType={customPieceCharSelected}
                     movementPattern={gameState.movement_patterns[customPieceCharSelected]}
        />
    </div>
{/if}
