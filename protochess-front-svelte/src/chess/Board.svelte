<script>
    import Tile from './Tile.svelte';
    import Piece from './Piece.svelte';
    import TileHighlight from "./TileHighlight.svelte";
    import {PlayersList} from '../WebsocketStore';
    import {sendTakeTurn, GameState} from "../WebsocketStore";
    let highlighted = null;
    let last_clicked = {type: "none"};
    $: gameDimensions = {width: $GameState.width, height: $GameState.height};
    $: tileDimensions = {width: 100 / $GameState.width, height: 100 / $GameState.height};

    function handlePieceClick(e) {
        let piece = e.detail;
        if (last_clicked.type === 'piece') {
            let from = [last_clicked.target.x, last_clicked.target.y];
            let to = [piece.x, piece.y];
            sendTakeTurn(from, to);
        }
        last_clicked = {type: 'piece', target: piece};
        highlighted = {x: piece.x, y: piece.y};
    }

    function handleTileClick(e) {
        let tile = e.detail;
        if (last_clicked.type === 'piece') {
            let from = [last_clicked.target.x, last_clicked.target.y];
            let to = [tile.x, tile.y];
            sendTakeTurn(from, to);
        }
        last_clicked = {type: "tile", target: tile};
    }
</script>

<style>
    #board{
        background-color: black;
        width: 100%;
        height: 100%;
    }
</style>

<div id="board">
    {#each $GameState.tiles as tile}
        <Tile on:tile_click={handleTileClick} {tile} flipped={$PlayersList.player_num !== 0 } {gameDimensions} {tileDimensions}/>
    {/each}
    {#each $GameState.pieces as piece}
        <Piece on:piece_click={handlePieceClick} {piece} flipped={$PlayersList.player_num !== 0} {gameDimensions} {tileDimensions}/>
    {/each}
    {#if highlighted }
        <TileHighlight pos={highlighted} flipped={$PlayersList.player_num !== 0} {gameDimensions} {tileDimensions}
        />
    {/if}}
</div>