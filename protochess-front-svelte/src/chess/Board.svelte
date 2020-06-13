<script>
    import Tile from './Tile.svelte';
    import Piece from './Piece.svelte';
    import ColorConstants from './ColorConstants';
    import { createEventDispatcher } from 'svelte';
    export let gameState;
    export let player_num;
    export let flipped;
    export let highlighted = null;
    const dispatch = createEventDispatcher();
    $: gameDimensions = {width: gameState.width, height: gameState.height};
    $: tileDimensions = {width: 100 / gameState.width, height: 100 / gameState.height};

    function handleTileClick(e) {
        let tile = e.detail;
        if (gameState.to_move === player_num) {
            //Set local highlight + show possible moves
            for (let pce of gameState.pieces) {
                if (pce.x === tile.x && pce.y === tile.y){
                    dispatch("gameRequest", {type: "MovesFrom", content: [tile.x, tile.y]});
                    break;
                }
            }
        }
    }

    function handleHighlightToClick(e){
        let tile = e.detail;
        let to = [tile.x, tile.y];
        console.log("here")
        console.log(highlighted)
        //If we click on a highted To square, we can send a move based on tile highlighting
        dispatch("gameRequest", {"content":{"from":highlighted.possibleMoves.from,
                "promote_to":null,"to":to}, type:"TakeTurn"});

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
    {#each gameState.tiles as tile}
        <Tile color = { tile.tile_type === 'b' ? '#a97d5d' : '#f7dcb4' }
              on:tileClick={handleTileClick} {tile} {flipped} {gameDimensions} {tileDimensions}/>
    {/each}
    {#if highlighted}
        {#if highlighted.lastTurn}
            <Tile color = {ColorConstants.FROM_HIGHLIGHT_COLOR}
                  on:tileClick={handleTileClick}
                  tile={{x: highlighted.lastTurn.from[0], y: highlighted.lastTurn.from[1]}}
                  {flipped} {gameDimensions} {tileDimensions}/>

            <Tile color = {ColorConstants.TO_HIGHLIGHT_COLOR}
                  on:tileClick={handleTileClick}
                  tile={{x: highlighted.lastTurn.to[0], y: highlighted.lastTurn.to[1]}}
                  {flipped} {gameDimensions} {tileDimensions}/>
        {/if}
        {#if highlighted.possibleMoves}
            <Tile color = {ColorConstants.POSSIBLE_FROM_HIGHLIGHT_COLOR}
                  on:tileClick={handleTileClick}
                  tile={{x: highlighted.possibleMoves.from[0], y: highlighted.possibleMoves.from[1]}}
                  {flipped} {gameDimensions} {tileDimensions}/>
            {#each highlighted.possibleMoves.to as pos}
                <Tile color = {ColorConstants.POSSIBLE_TO_HIGHLIGHT_COLOR}
                      on:tileClick={handleHighlightToClick} tile={{x: pos[0], y: pos[1]}}
                      {flipped} {gameDimensions} {tileDimensions}/>
            {/each}
        {/if}
        {#if highlighted.in_check_kings}
            {#each highlighted.in_check_kings as piece}
                <Tile color = {ColorConstants.IN_CHECK_HIGHLIGHT_COLOR}
                      on:tileClick={handleTileClick} tile={{x: piece.x, y: piece.y}}
                      {flipped} {gameDimensions} {tileDimensions}/>
            {/each}
        {/if}
    {/if}

    {#each gameState.pieces as piece}
    <div style="pointer-events: none">
        <Piece {piece} {flipped} {gameDimensions} {tileDimensions}/>
    </div>
    {/each}
</div>