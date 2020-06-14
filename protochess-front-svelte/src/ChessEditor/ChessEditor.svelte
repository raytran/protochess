<script>
    import Tile from '../chess/Tile.svelte'
    import {onMount} from 'svelte';
    import Board from "../chess/Board.svelte";
    import PieceEditor from "./PieceEditor.svelte";

    onMount(async () => {

    });

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

    let pieceCharSelected = 'a';
    let customChars = 'acdefghijlmostuvwxyz'.split('');

    let pieceSelected = PieceType.PAWN;
    let pieceOwnerSelected = 0;
    let toolSelected = ToolType.TILE;

    let gameState = {width: 8, height: 8, tiles: [{x: 0, y: 0, tile_type: 'b'}], pieces: []}
    let dimensions = {width: 8, height: 8};

    $: gameStateJSON = JSON.stringify(gameState);

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
        gameState = {width: 8, height: 8, tiles: [{x: 0, y: 0, tile_type: 'b'}], pieces: []};
        dimensions = {width: 8, height: 8};
        restricted = [];
    }

    let clicked = false;

    function toggleRestricted(tile) {
        console.log("we in here bro");
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
                    piece_type: pieceCharSelected
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
</script>

<style>
    input{
        margin:0.4em;
    }
    fieldset{
        -webkit-box-shadow: 0px 15px 20px -8px rgba(0,0,0,0.4);
        -moz-box-shadow: 0px 15px 20px -8px rgba(0,0,0,0.4);
        box-shadow: 0px 15px 20px -8px rgba(0,0,0,0.4);
        padding: 2em;
        border: 0;
        text-align: left;
    }

    #boardWrapper{
        -webkit-box-shadow: 0px 15px 20px -8px rgba(0,0,0,0.4);
        -moz-box-shadow: 0px 15px 20px -8px rgba(0,0,0,0.4);
        box-shadow: 0px 15px 20px -8px rgba(0,0,0,0.4);
    }


    .dropdown-content {
        display: none;
    }

    #customPieceDropdown:hover .dropdown-content {
        display: block;
    }


</style>





<div style="justify-content:center; font-size: 1.3em; display: flex; flex-direction: row">
    <fieldset style="margin-right: 2em">
        <button on:click={reset}>Reset</button>
        <button on:click={()=>console.log("bruh")}>Save Changes</button>
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
            <textarea style="height: 10em" readonly value={gameStateJSON}></textarea>
        </label>

    </fieldset>

    <div id=boardWrapper style="position: relative; width: 35em; height: 35em" on:mouseleave={() => clicked = false}>
        <Board
                {flipped}
                player_num={0}
                gameState={gameState}
                on:tileMouseUp={()=> clicked = false}
                on:tileMouseOver={e => (clicked) ?  activateTool(e.detail) : ""}
                        on:tileMouseDown={e => {clicked = true; activateTool(e.detail);}} />
    </div>
    <fieldset style="margin-left: 2em; padding-right: 3em;">
        <legend style="float: left"><b>Select Tool</b></legend>
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
                <div id="customPieceDropdown">
                    Custom Piece:
                    <ul class='dropdown-content' style="height: 8em;overflow: auto">
                        {#each customChars as char}
                            <li>{char}</li>
                        {/each}
                    </ul>
                </div>
            {/if}
        {/if}
    </fieldset>

    <div style="position: absolute; left: 0; top:0;">
        <PieceEditor selectedPiece="p"/>
    </div>

</div>
