<script>
    import Modal from '../Modal.svelte'
    import {createEventDispatcher, onMount} from 'svelte';
    import { Tabs, Tab } from 'svelma'
    import MediaQuery from "svelte-media-query";
    import Board from "../Chess/Board.svelte";
    import PieceEditor from "./PieceEditor.svelte";
    import ChessLeftControl from "./ChessLeftControl.svelte";
    import ChessRightControl from "./ChessRightControl.svelte";

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

    $: gameStateJSON = (() => {
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

    function onSaveChanges() {
        dispatch("saveChanges", gameState);
    }
</script>

<style>
    .leftControl{
        max-width:400px;
        width: 100%;
        padding: 1em;
        text-align: center;
    }

    .boardWrapper {
        max-width: 700px;
        width: 100%;
    }

    .rightControl {
        max-width:400px;
        width: 100%;
        padding: 1em;
    }
</style>


<MediaQuery query="(min-width: 1400px)" let:matches>
    {#if matches}
        <div class="container">
            <div class="columns">
                <div class="column box leftControl">
                    <ChessLeftControl
                            bind:gameState={gameState}
                            bind:flipped={flipped}
                            bind:dimensions={dimensions}
                            {gameStateJSON}
                            on:reset={reset}
                            on:saveChanges={onSaveChanges}/>
                </div>
                <div class="column boardWrapper">
                    <Board
                            {flipped}
                            player_num={0}
                            gameState={gameState}
                            on:tileMouseUp={()=> clicked = false}
                            on:tileMouseOver={e => (clicked) ?  activateTool(e.detail) : ""}
                                    on:tileMouseDown={e => {clicked = true; activateTool(e.detail);}} />
                </div>
                <div class="column box rightControl">
                    <ChessRightControl
                            bind:toolSelected={toolSelected}
                            bind:pieceOwnerSelected={pieceOwnerSelected}
                            bind:customPieceCharSelected={customPieceCharSelected}
                            bind:pieceSelected={pieceSelected}
                            bind:registeredChars={registeredChars}
                            bind:unregisteredChars={unregisteredChars}
                            on:showPieceEditor={()=> showPieceEditor = true}
                    />
                </div>
            </div>
        </div>
    {:else}
        <div class="container" style="margin-bottom:1em; display: flex; justify-content: center">
            <div>
                <button class="button is-danger" on:click={reset}>Reset</button>
                <button class="button is-primary" on:click={onSaveChanges}>Save Changes</button>
            </div>
        </div>
        <Tabs position="is-centered" style="is-toggle is-medium">
            <Tab label="Board">
                <div class="container boardWrapper" style="height:100vh">
                    <Board
                            {flipped}
                            player_num={0}
                            gameState={gameState}
                            on:tileMouseUp={()=> clicked = false}
                            on:tileMouseOver={e => (clicked) ?  activateTool(e.detail) : ""}
                                    on:tileMouseDown={e => {clicked = true; activateTool(e.detail);}} />
                </div>
            </Tab>
            <Tab label="Controls">
                <div class="columns" style="height: 100vh; overflow:auto ">
                    <div class="box column rightControl">
                        <ChessRightControl
                                bind:toolSelected={toolSelected}
                                bind:pieceOwnerSelected={pieceOwnerSelected}
                                bind:customPieceCharSelected={customPieceCharSelected}
                                bind:pieceSelected={pieceSelected}
                                bind:registeredChars={registeredChars}
                                bind:unregisteredChars={unregisteredChars}
                                on:showPieceEditor={()=> showPieceEditor = true}
                        />
                    </div>
                    <div class="box column leftControl">
                        <ChessLeftControl
                                bind:gameState={gameState}
                                bind:flipped={flipped}
                                bind:dimensions={dimensions}
                                {gameStateJSON}
                                on:reset={reset}
                                showSaveChanges={false}
                                on:saveChanges={onSaveChanges}/>
                    </div>
                </div>
            </Tab>
        </Tabs>
    {/if}
</MediaQuery>



{#if showPieceEditor}
    <Modal on:close={()=>showPieceEditor = false} >
        <div slot="header" style="text-align: center">
            <h1 class="title">Piece Editor</h1>
        </div>
        <div>
            <PieceEditor on:saveChanges={handlePieceEditor}
                         pieceType={customPieceCharSelected}
                         movementPattern={gameState.movement_patterns[customPieceCharSelected]}
            />
        </div>
    </Modal>
{/if}
