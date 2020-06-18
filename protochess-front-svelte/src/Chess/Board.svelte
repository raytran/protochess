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
    function handleTileClick(e) {
        dispatch("tileClick", e.detail);
        let tile = e.detail;
        if (gameState.to_move === player_num) {
            //Set local highlight + show possible moves
            for (let pce of gameState.pieces) {
                if (pce.owner === player_num && pce.x === tile.x && pce.y === tile.y){
                    dispatch("gameRequest", {type: "MovesFrom", content: [tile.x, tile.y]});
                    break;
                }
            }
        }
    }

    function handleHighlightToClick(e){
        dispatch("tileClick", e.detail);
        let tile = e.detail;
        let to = [tile.x, tile.y];
        //If we click on a highted To square, we can send a move based on tile highlighting
        dispatch("gameRequest", {"content":{"from":highlighted.possibleMoves.from,
                "promote_to":null,"to":to}, type:"TakeTurn"});

    }
</script>

<style>
    #boardWrapper{
        position: relative;
        width: 100%;
        height: 0;
        padding-bottom: 100%;
    }
    #board{
        background-color: #EAEAEA;
        grid-template-columns: repeat(var(--size), 1fr);
        grid-template-rows: repeat(var(--size), 1fr);

        box-shadow: 0 14px 28px rgba(0,0,0,0.25), 0 10px 10px rgba(0,0,0,0.22);

        display: grid;
        justify-items: center;
    }
</style>

<div id="boardWrapper">
    <div id="board" style="--size: {Math.max(gameState.width, gameState.height)}">
        {#if gameState.tiles}
            {#each gameState.tiles as tile}
                <Tile color = { tile.tile_type === 'b' ? ColorConstants.DARK_SQUARE : tile.tile_type === 'w' ? ColorConstants.LIGHT_SQUARE : ColorConstants.DISABLED }
                      on:tileClick={handleTileClick}
                      on:tileMouseOver
                      on:tileMouseDown
                      on:tileMouseUp
                      {tile} {flipped} gameHeight={gameState.height} gameWidth={gameState.width} />
            {/each}
        {/if}
        {#if highlighted}
            {#if highlighted.lastTurn}
                <Tile color = {ColorConstants.FROM_HIGHLIGHT_COLOR}
                      on:tileClick={handleTileClick}
                      on:tileMouseOver
                      on:tileMouseDown
                      on:tileMouseUp
                      tile={{x: highlighted.lastTurn.from[0], y: highlighted.lastTurn.from[1]}}
                      {flipped}
                      gameHeight={gameState.height} gameWidth={gameState.width} />

                <Tile color = {ColorConstants.TO_HIGHLIGHT_COLOR}
                      on:tileClick={handleTileClick}
                      on:tileMouseOver
                      on:tileMouseDown
                      on:tileMouseUp
                      tile={{x: highlighted.lastTurn.to[0], y: highlighted.lastTurn.to[1]}}
                      {flipped}
                      gameHeight={gameState.height} gameWidth={gameState.width} />
            {/if}
            {#if highlighted.possibleMoves}
                <Tile color = {ColorConstants.POSSIBLE_FROM_HIGHLIGHT_COLOR}
                      on:tileClick={handleTileClick}
                      on:tileClick={handleTileClick}
                      on:tileMouseOver
                      on:tileMouseDown
                      on:tileMouseUp
                      tile={{x: highlighted.possibleMoves.from[0], y: highlighted.possibleMoves.from[1]}}
                      {flipped}
                      gameHeight={gameState.height} gameWidth={gameState.width} />
                {#each highlighted.possibleMoves.to as pos}
                    <Tile color = {ColorConstants.POSSIBLE_TO_HIGHLIGHT_COLOR}
                          on:tileClick={handleHighlightToClick} tile={{x: pos[0], y: pos[1]}}
                          {flipped}
                          gameHeight={gameState.height} gameWidth={gameState.width} />
                {/each}
            {/if}
            {#if highlighted.in_check_kings}
                {#each highlighted.in_check_kings as piece}
                    <Tile color = {ColorConstants.IN_CHECK_HIGHLIGHT_COLOR}
                          on:tileClick={handleTileClick} tile={{x: piece.x, y: piece.y}}
                          on:tileMouseOver
                          on:tileMouseDown
                          on:tileMouseUp
                          {flipped}
                          gameHeight={gameState.height} gameWidth={gameState.width} />
                {/each}
            {/if}
            {#if highlighted.etc}
                {#each highlighted.etc as etc}
                    <Tile color = {etc.color}
                          on:tileClick={handleTileClick} tile={{x: etc.x, y: etc.y}}
                          on:tileMouseOver
                          on:tileMouseDown
                          on:tileMouseUp
                          {flipped}
                          gameHeight={gameState.height} gameWidth={gameState.width} />
                {/each}
            {/if}
        {/if}
        {#if gameState.pieces}
            {#each gameState.pieces as piece}
                <Piece on:pieceClick {piece} {flipped}
                       gameHeight={gameState.height} gameWidth={gameState.width} />
            {/each}
        {/if}
    </div>
</div>
