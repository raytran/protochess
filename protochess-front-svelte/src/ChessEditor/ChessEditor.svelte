<script>
    import Tile from '../chess/Tile.svelte'
    import Board from "../chess/Board.svelte";

    let gameState = {width: 8, height:8, tiles: [{x: 0, y:0, tile_type:'b'}]}
    let dimensions = {width: 8, height:8};

    let restricted = [];
    let flipped = false;
    $: gameState.width = dimensions.width;
    $: gameState.height = dimensions.height;
    $: gameState.tiles = (() =>{
        let tiles = [];
        for (let x=0;x<dimensions.width;x++){
            for (let y=0;y<dimensions.height;y++){
                let allowed = true;
                for (let r of restricted)  {
                    if (r.x === x && r.y === y) {
                        allowed = false;
                        break;
                    }
                }
                if (allowed){
                    tiles.push({x:x, y:y, tile_type: (x+y) % 2 === 0 ? 'b':'w'});
                }else{
                    tiles.push({x:x, y:y, tile_type: 'x'});
                }
            }
        }
        return tiles;
    })();

    function flipRestricted(tile){
        console.log("we in here bro");
        let i = restricted.findIndex(obj => obj.x === tile.x &&  obj.y === tile.y );
        if (i !== -1){
            //Not here, insert
            restricted = restricted.filter(item => !(item.x === tile.x && item.y === tile.y));
        }else{
            restricted = [...restricted, {x: tile.x, y:tile.y}];
        }
    }
    function reset() {
        gameState = {width: 8, height:8, tiles: [{x: 0, y:0, tile_type:'b'}]}
        dimensions = {width: 8, height:8};
        restricted = [];
    }


    let clicked = false;
    function handleTileMouseOver(e){
        if (clicked){
            let tile = e.detail;
            let i = restricted.findIndex(obj => obj.x === tile.x &&  obj.y === tile.y );
            if (i !== -1){
                //Not here, insert
                restricted = restricted.filter(item => !(item.x === tile.x && item.y === tile.y));
            }else{
                restricted = [...restricted, {x: tile.x, y:tile.y}];
            }
        }
    }

    function handleTileClick(e){
        clicked = false;
        flipRestricted(e.detail);
    }

    function handleMouseUp(e){
        console.log("mouse up");
        clicked = false;
    }
</script>





<div style="position: relative; width: 30em; height: 30em">
    <Board
            on:tileClick={handleTileClick}
            on:tileMouseOver={handleTileMouseOver}
            on:tileMouseDown={() => clicked = true}
            on:tileMouseUp={handleMouseUp}
            gameState={gameState}
            player_num={0}
            {flipped} />
</div>

Board Width
<label>
    <input type=number bind:value={dimensions.width} min=1 max=16>
    <input type=range bind:value={dimensions.width} min=1 max=16>
</label>

Board Height
<label>
    <input type=number bind:value={dimensions.height} min=1 max=16>
    <input type=range bind:value={dimensions.height} min=1 max=16>
</label>

<label>
    <input type=checkbox bind:checked = {flipped}>
    View as black:
</label>

<button on:click={reset}>Reset</button>
<button on:click={()=>console.log("bruh")}>Save Changes</button>

